// flutter_rust_bridge が生成する frb_generated.rs は
// `use crate::api::wrapper::*;` でこのモジュールを glob import するため、
// API のシグネチャに現れる型は pub use で再エクスポートしておく必要がある。
// なお std::path::Path は DST で opaque 型にできないため PathBuf を使う。
pub use std::{path::PathBuf, sync::Mutex};

pub use btc_wallet::{BtcWallet, Config};

use log::*;
use wallet_utils::encdec;

// BtcWallet は内部に rusqlite::Connection (RefCell) を持つため Sync ではなく、
// flutter_rust_bridge の opaque 型に直接できない。Mutex で包んで渡す
// (FRB は Mutex<T> を透過的に扱い、Dart 側では T として見える)。
use std::{str::FromStr, sync::Mutex as _Mutex};

pub struct WalletWrapper(_Mutex<BtcWallet>);

pub struct SendResult {
    pub tx: String,
    pub txid: String,
}

impl WalletWrapper {
    pub fn create_wallet(
        network: &str,
        electrum_server: &str,
        passphrase: &str,
        wallet_path: String,
    ) -> anyhow::Result<Self> {
        trace!("create_wallet: network={}, electrum_server={}", network, electrum_server);
        let wallet_path = PathBuf::from(wallet_path);
        let electrum_config = btc_wallet::ElectrumConfig {
            enabled: true,
            server: electrum_server.into(),
            batch_size: 30,
            gap_limit: 20,
        };
        let config = btc_wallet::Config {
            network: btc_wallet::Network::from_str(network).map_err(anyhow::Error::msg)?,
            electrum: electrum_config,
            backend: btc_wallet::Backend::Electrum,
        };
        let (wallet, xprv) = BtcWallet::create(config, &wallet_path)?;
        store_xprv(passphrase, &wallet_path, &xprv, network, electrum_server)?;
        trace!("Wallet created successfully");
        Ok(Self(_Mutex::new(wallet)))
    }

    pub fn load_wallet(passphrase: &str, wallet_path: String) -> anyhow::Result<Self> {
        info!("load_wallet: wallet_path={}", wallet_path);
        let wallet_path = PathBuf::from(wallet_path);
        let (xprv, config) = load_xprv(passphrase, &wallet_path)?;
        let wallet = BtcWallet::load(config, &xprv, &wallet_path)?;
        info!("Wallet loaded successfully");
        Ok(Self(_Mutex::new(wallet)))
    }

    pub fn balance(&mut self) -> anyhow::Result<u64> {
        let mut wallet = self.0.lock().unwrap();
        wallet.sync()?;
        trace!("wallet synced");
        let balance = wallet.balance();
        trace!("Current balance: {:?}", balance);
        Ok(balance.confirmed.to_sat())
    }

    pub fn new_address(&mut self) -> anyhow::Result<String> {
        let mut wallet = self.0.lock().unwrap();
        let address = wallet.new_address()?;
        trace!("New address generated: {}", address);
        Ok(address.to_string())
    }

    pub fn send_tx(
        &mut self,
        out_addr: &str,
        amount: u64,
        fee_rate: f64,
    ) -> anyhow::Result<SendResult> {
        let mut wallet = self.0.lock().unwrap();
        let address = wallet.parse_address(out_addr)?;
        let tx = wallet.create_tx(&address, amount, fee_rate)?;
        let txid = wallet.send_tx(&tx)?;
        Ok(SendResult {
            tx: btc_wallet::to_tx_hex(&tx),
            txid: txid.to_string(),
        })
    }

    pub fn send_tx_single_anypay(
        &mut self,
        out_addr: &str,
        amount: u64,
        fee_rate: f64,
    ) -> anyhow::Result<SendResult> {
        let mut wallet = self.0.lock().unwrap();
        let address = wallet.parse_address(out_addr)?;
        let tx = wallet.create_tx_single_anypay(&address, amount, fee_rate)?;
        let txid = wallet.send_tx(&tx)?;
        Ok(SendResult {
            tx: btc_wallet::to_tx_hex(&tx),
            txid: txid.to_string(),
        })
    }
}

fn store_xprv(
    passphrase: &str,
    wallet_path: &PathBuf,
    xprv: &str,
    network: &str,
    electrum_server: &str,
) -> anyhow::Result<()> {
    let v = encdec::encode_private_key(xprv, passphrase)?;

    let conn = rusqlite::Connection::open(wallet_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wallet_key(
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            private_key BLOB NOT NULL,
            network TEXT NOT NULL,
            electrum_server TEXT NOT NULL)
        ",
        (),
    )?;
    conn.execute(
        "INSERT INTO wallet_key (private_key, network, electrum_server) VALUES (?1, ?2, ?3)",
        (v, network, electrum_server),
    )?;
    Ok(())
}

fn load_xprv(
    passphrase: &str,
    wallet_path: &PathBuf,
) -> anyhow::Result<(String, btc_wallet::Config)> {
    let conn = rusqlite::Connection::open(wallet_path)?;
    let result: (Vec<u8>, String, String) = conn.query_row(
        "SELECT private_key, network, electrum_server FROM wallet_key LIMIT 1",
        (),
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    let xprv = encdec::decode_private_key(&result.0, passphrase)?;
    let config = btc_wallet::Config {
        network: btc_wallet::Network::from_str(&result.1).map_err(anyhow::Error::msg)?,
        electrum: btc_wallet::ElectrumConfig {
            enabled: true,
            server: result.2,
            batch_size: 30,
            gap_limit: 20,
        },
        backend: btc_wallet::Backend::Electrum,
    };
    Ok((xprv, config))
}

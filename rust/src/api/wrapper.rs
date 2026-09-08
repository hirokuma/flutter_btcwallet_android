// flutter_rust_bridge が生成する frb_generated.rs は
// `use crate::api::wrapper::*;` でこのモジュールを glob import するため、
// API のシグネチャに現れる型は pub use で再エクスポートしておく必要がある。
// なお std::path::Path は DST で opaque 型にできないため PathBuf を使う。
pub use std::path::PathBuf;
pub use std::sync::Mutex;

pub use btc_wallet::{BtcWallet, Config};

use wallet_utils::encdec;

// BtcWallet は内部に rusqlite::Connection (RefCell) を持つため Sync ではなく、
// flutter_rust_bridge の opaque 型に直接できない。Mutex で包んで渡す
// (FRB は Mutex<T> を透過的に扱い、Dart 側では T として見える)。
use std::{str::FromStr, sync::Mutex as _Mutex};

pub fn create_wallet(
    network: &str,
    electrum_server: &str,
    passphrase: &str,
    wallet_path: String,
) -> anyhow::Result<Mutex<BtcWallet>> {
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
    Ok(_Mutex::new(wallet))
}

pub fn load_wallet(passphrase: &str, wallet_path: String) -> anyhow::Result<Mutex<BtcWallet>> {
    let wallet_path = PathBuf::from(wallet_path);
    let (xprv, config) = load_xprv(passphrase, &wallet_path)?;
    Ok(_Mutex::new(BtcWallet::load(config, &xprv, &wallet_path)?))
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

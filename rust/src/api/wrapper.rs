// flutter_rust_bridge が生成する frb_generated.rs は
// `use crate::api::wrapper::*;` でこのモジュールを glob import するため、
// API のシグネチャに現れる型は pub use で再エクスポートしておく必要がある。
// なお std::path::Path は DST で opaque 型にできないため PathBuf を使う。
pub use btc_wallet::{BtcWallet, Config};
pub use std::path::PathBuf;
pub use std::sync::Mutex;

// BtcWallet は内部に rusqlite::Connection (RefCell) を持つため Sync ではなく、
// flutter_rust_bridge の opaque 型に直接できない。Mutex で包んで渡す
// (FRB は Mutex<T> を透過的に扱い、Dart 側では T として見える)。
use std::sync::Mutex as _Mutex;

pub fn create_wallet(
    config: Config,
    wallet_path: PathBuf,
) -> anyhow::Result<(Mutex<BtcWallet>, String)> {
    let (wallet, xprv) = BtcWallet::create(config, &wallet_path)?;
    Ok((_Mutex::new(wallet), xprv))
}

pub fn load_wallet(config: Config, xprv: &str, wallet_path: PathBuf) -> anyhow::Result<Mutex<BtcWallet>> {
    Ok(_Mutex::new(BtcWallet::load(config, xprv, &wallet_path)?))
}

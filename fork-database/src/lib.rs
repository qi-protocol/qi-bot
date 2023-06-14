pub mod backend_handler;
pub mod blockchain_db;
pub mod errors;
pub mod forked_db;
pub mod shared_backend;
pub mod snapshot;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain_db::{BlockchainDb, BlockchainDbMeta, JsonBlockCacheDB};
    use crate::shared_backend::SharedBackend;
    use revm::db::DatabaseRef;
    use revm::primitives::{B160, U256 as rU256};

    use dotenv::dotenv;
    use ethers::providers::{Provider, Ws};
    use ethers::types::Chain;
    use foundry_config::Config;
    use foundry_evm::executor::opts::EvmOpts;
    use parking_lot::RwLock;
    use std::env;
    use std::{collections::BTreeSet, path::PathBuf, sync::Arc,};

    async fn setup() -> (Provider<Ws>, String) {
        dotenv().ok();

        let _blast_key = env::var("BLAST_API_KEY").unwrap();
        let mainnet_blast_url = format!("wss://eth-mainnet.blastapi.io/{}", _blast_key);

        let provider = Provider::<Ws>::connect(mainnet_blast_url.clone())
            .await
            .ok()
            .unwrap();

        (provider, mainnet_blast_url)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_blockchaindb_shared_backend_syncing() {
        let (provider, mainnet_blast_url) = setup().await;

        let meta = BlockchainDbMeta {
            cfg_env: Default::default(),
            block_env: Default::default(),
            hosts: BTreeSet::from([mainnet_blast_url.clone().to_string()]),
        };

        let db = BlockchainDb::new(meta, None);
        let backend = SharedBackend::spawn_backend(Arc::new(provider), db.clone(), None).await;

        // vitalik's address
        let address: B160 = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045 "
            .parse()
            .unwrap();

        let idx = rU256::from(0u64);
        let value = backend.storage(address, idx).unwrap();
        let account = backend.basic(address).unwrap().unwrap();

        // test accounts
        let mem_acc = db.accounts().read().get(&address).unwrap().clone();
        assert_eq!(account.balance, mem_acc.balance);
        assert_eq!(account.nonce, mem_acc.nonce);

        // test storage
        let slots = db.storage().read().get(&address).unwrap().clone();
        assert_eq!(slots.len(), 1);
        assert_eq!(slots.get(&idx).copied().unwrap(), value);

        // test hash
        let num = rU256::from(10u64);
        let hash = backend.block_hash(num).unwrap();
        let mem_hash = *db.block_hashes().read().get(&num).unwrap();
        assert_eq!(hash, mem_hash);

        let handle = std::thread::spawn(move || {
            for i in 1..10 {
                let idx = rU256::from(i);
                let _ = backend.storage(address, idx);
            }
        });
        handle.join().unwrap();
        let slots = db.storage().read().get(&address).unwrap().clone();
        assert_eq!(slots.len() as u64, 10);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_flush_and_read_cache() {
        let (provider, mainnet_blast_url) = setup().await;

        let cache_path = PathBuf::from("src/cache_data/storage.json");

        let meta = BlockchainDbMeta {
            cfg_env: Default::default(),
            block_env: Default::default(),
            hosts: BTreeSet::from([mainnet_blast_url.clone().to_string()]),
        };

        let db = BlockchainDb::new(meta, Some(cache_path.clone()));
        let backend = SharedBackend::spawn_backend(Arc::new(provider), db.clone(), None).await;

        let address: B160 = "0xd8da6bf26964af9d7eed9e03e53415d37aa96045 "
            .parse()
            .unwrap();

        let idx = rU256::from(0u64);
        let _ = backend.storage(address, idx).unwrap();
        let _ = backend.basic(address).unwrap().unwrap();
        let _ = db.accounts().read().get(&address).unwrap().clone();

        // write to cache
        let _ = db.cache().flush();

        // read from cache
        let json = JsonBlockCacheDB::load(cache_path).unwrap();
        assert!(!json.db().accounts.read().is_empty());
    }
}

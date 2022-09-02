//! An in-memory [`Store`] implementation. Useful for testing.
//!
//! # Example Usage
//! ```rust
//! use rcommunity_core::store::memory::MemoryStore;
//! use rcommunity_core::store::{Store, Transaction};
//!
//! tokio_test::block_on(async {
//!     let mut store = MemoryStore::default();
//!     let mut txn = store.begin_txn().await.unwrap();
//!     assert!(txn.get(b"key").await.unwrap().is_none());
//!     txn.put(b"key", b"value").await.unwrap();
//!     assert_eq!(
//!         txn.get(b"key").await.unwrap().unwrap(),
//!         b"value",
//!     );
//! })
//! ```

use async_trait::async_trait;
use parking_lot::{Condvar, Mutex, MutexGuard};
use std::{collections::BTreeMap, sync::Arc};

use crate::error::Result;

use super::{Store, Transaction};

type ByteMap = BTreeMap<Vec<u8>, Vec<u8>>;

/// Implementation of an in-memory [`Store`].
///
/// Internally, each operation of `MemoryStore` first obtains a transaction lock, then locks the
/// mutex protecting the underlying store. The transaction lock is implemented as a [`Condvar`],
/// checking whether it is currently in the same transaction (or no transaction at all) before
/// entering.
#[derive(Debug, Default)]
pub struct MemoryStore {
    store: Arc<Mutex<ByteMap>>,
    cur_txn_id: Arc<(Mutex<usize>, Condvar)>,
    max_txn_id: usize,
}

#[async_trait]
impl Store for MemoryStore {
    type Transaction = MemoryTransaction;
    async fn begin_txn(&mut self) -> Result<MemoryTransaction> {
        self.max_txn_id += 1;
        Ok(MemoryTransaction {
            store: self.store.clone(),
            cur_txn_id: self.cur_txn_id.clone(),
            id: self.max_txn_id,
        })
    }
}

/// Transaction type for [`MemoryStore`].
pub struct MemoryTransaction {
    store: Arc<Mutex<ByteMap>>,
    cur_txn_id: Arc<(Mutex<usize>, Condvar)>,
    id: usize,
}

impl MemoryTransaction {
    fn txn_lock(&self) -> (MutexGuard<usize>, &Condvar) {
        let &(ref lock, ref cvar) = &*self.cur_txn_id;
        let mut cur_txn_id = lock.lock();
        if *cur_txn_id != 0 && *cur_txn_id != self.id {
            cvar.wait(&mut cur_txn_id);
        }
        (cur_txn_id, cvar)
    }
    fn release_txn_lock(&self) {
        let &(ref lock, ref cvar) = &*self.cur_txn_id;
        let mut cur_txn_id = lock.lock();
        if *cur_txn_id == self.id {
            *cur_txn_id = 0;
            cvar.notify_one();
        }
    }
}

#[async_trait]
impl Transaction for MemoryTransaction {
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let (cur_txn_id, cvar) = self.txn_lock();
        let value = self.store.lock().get(key).cloned();
        if *cur_txn_id == 0 {
            cvar.notify_one();
        }
        Ok(value)
    }

    async fn get_for_update(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let (mut cur_txn_id, _) = self.txn_lock();
        *cur_txn_id = self.id;
        Ok(self.store.lock().get(key).cloned())
    }

    async fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        let (cur_txn_id, cvar) = self.txn_lock();
        self.store.lock().insert(key.to_vec(), value.to_vec());
        if *cur_txn_id == 0 {
            cvar.notify_one();
        }
        Ok(())
    }

    async fn scan(
        &self,
        start: &[u8],
        end: &[u8],
        limit: usize,
    ) -> Result<Box<dyn Iterator<Item = (Vec<u8>, Vec<u8>)>>> {
        let (cur_txn_id, cvar) = self.txn_lock();
        // needs collect here to pass across async boundary
        #[allow(clippy::needless_collect)]
        let value: Vec<(Vec<u8>, Vec<u8>)> = self
            .store
            .lock()
            .range(start.to_vec()..end.to_vec())
            .take(limit)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        if *cur_txn_id == 0 {
            cvar.notify_one();
        }
        Ok(Box::new(value.into_iter()))
    }

    async fn scan_keys(
        &self,
        start: &[u8],
        end: &[u8],
        limit: usize,
    ) -> Result<Box<dyn Iterator<Item = Vec<u8>>>> {
        let (cur_txn_id, cvar) = self.txn_lock();
        // needs collect here to pass across async boundary
        #[allow(clippy::needless_collect)]
        let value: Vec<Vec<u8>> = self
            .store
            .lock()
            .range(start.to_vec()..end.to_vec())
            .take(limit)
            .map(|(k, _)| k.clone())
            .collect();
        if *cur_txn_id == 0 {
            cvar.notify_one();
        }
        Ok(Box::new(value.into_iter()))
    }

    async fn delete(&mut self, key: &[u8]) -> Result<()> {
        let (cur_txn_id, cvar) = self.txn_lock();
        self.store.lock().remove(key);
        if *cur_txn_id == 0 {
            cvar.notify_one();
        }
        Ok(())
    }

    async fn commit(&mut self) -> Result<()> {
        self.release_txn_lock();
        Ok(())
    }
}

impl Drop for MemoryTransaction {
    /// Ensure transaction lock is released before dropping.
    fn drop(&mut self) {
        self.release_txn_lock();
    }
}

#[cfg(test)]
mod test {
    use crate::store::{Store, Transaction};

    use super::MemoryStore;

    #[tokio::test]
    async fn test_memory_store() {
        let mut store = MemoryStore::default();
        {
            let mut txn = store.begin_txn().await.unwrap();
            assert!(txn.get(b"key").await.unwrap().is_none());
            txn.put(b"key", b"value").await.unwrap();
            assert_eq!(txn.get(b"key").await.unwrap().unwrap(), b"value");
        }
        {
            let mut txn = store.begin_txn().await.unwrap();
            assert_eq!(txn.get_for_update(b"key").await.unwrap().unwrap(), b"value");
            txn.put(b"key", b"").await.unwrap();
            assert_eq!(txn.get(b"key").await.unwrap().unwrap(), b"");
            txn.commit().await.unwrap();
        }
        {
            let mut txn = store.begin_txn().await.unwrap();
            txn.put(b"key2", b"v2").await.unwrap();
            assert!(
                txn.scan(b"key", b"key3", 10)
                    .await
                    .unwrap()
                    .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
                    == vec![
                        (b"key".to_vec(), b"".to_vec()),
                        (b"key2".to_vec(), b"v2".to_vec())
                    ]
            );
            assert!(
                txn.scan(b"key", b"key3", 1)
                    .await
                    .unwrap()
                    .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
                    == vec![(b"key".to_vec(), b"".to_vec())]
            );
            txn.put(b"key4", b"v4").await.unwrap();
            assert!(
                txn.scan_keys(b"key", b"key3", 10)
                    .await
                    .unwrap()
                    .collect::<Vec<Vec<u8>>>()
                    == vec![b"key".to_vec(), b"key2".to_vec()]
            );
        }
        {
            let mut txn = store.begin_txn().await.unwrap();
            assert_eq!(txn.get(b"key").await.unwrap().unwrap(), b"");
            txn.delete(b"key").await.unwrap();
            assert!(txn.get(b"key").await.unwrap().is_none());
        }
    }
}

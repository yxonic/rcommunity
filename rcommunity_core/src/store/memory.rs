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
//!     assert!(txn.get("key".into()).await.unwrap().is_none());
//!     txn.put("key".into(), "value".into()).await.unwrap();
//!     assert_eq!(
//!         txn.get("key".into()).await.unwrap().unwrap(),
//!         String::from("value")
//!     );
//! })
//! ```

use async_trait::async_trait;
use parking_lot::{Condvar, Mutex, MutexGuard};
use std::{collections::BTreeMap, sync::Arc};

use crate::error::Result;

use super::{Store, Transaction};

type StringMap = BTreeMap<String, String>;

/// Implementation of an in-memory [`Store`].
///
/// Internally, each operation of `MemoryStore` first obtains a transaction lock, then locks the
/// mutex protecting the underlying store. The transaction lock is implemented as a [`Condvar`],
/// checking whether it is currently in the same transaction (or no transaction at all) before
/// entering.
#[derive(Debug, Default)]
pub struct MemoryStore {
    store: Arc<Mutex<StringMap>>,
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
    store: Arc<Mutex<StringMap>>,
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
    async fn get(&self, key: String) -> Result<Option<String>> {
        let (cur_txn_id, cvar) = self.txn_lock();
        let value = self.store.lock().get(&key).map(String::from);
        if *cur_txn_id == 0 {
            cvar.notify_one();
        }
        Ok(value)
    }
    async fn get_for_update(&mut self, key: String) -> Result<Option<String>> {
        let (mut cur_txn_id, _) = self.txn_lock();
        *cur_txn_id = self.id;
        Ok(self.store.lock().get(&key).map(String::from))
    }
    async fn put(&mut self, key: String, value: String) -> Result<()> {
        let (cur_txn_id, cvar) = self.txn_lock();
        self.store.lock().insert(key, value);
        if *cur_txn_id == 0 {
            cvar.notify_one();
        }
        Ok(())
    }
    async fn delete(&mut self, key: String) -> Result<()> {
        let (cur_txn_id, cvar) = self.txn_lock();
        self.store.lock().remove(&key);
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
            assert!(txn.get("key".into()).await.unwrap().is_none());
            txn.put("key".into(), "value".into()).await.unwrap();
            assert_eq!(
                txn.get("key".into()).await.unwrap().unwrap(),
                String::from("value")
            );
        }
        {
            let mut txn = store.begin_txn().await.unwrap();
            assert_eq!(
                txn.get_for_update("key".into()).await.unwrap().unwrap(),
                String::from("value")
            );
            txn.put("key".into(), "".into()).await.unwrap();
            assert_eq!(
                txn.get("key".into()).await.unwrap().unwrap(),
                String::from("")
            );
            txn.commit().await.unwrap();
        }
        {
            let mut txn = store.begin_txn().await.unwrap();
            assert_eq!(
                txn.get("key".into()).await.unwrap().unwrap(),
                String::from("")
            );
            txn.delete("key".into()).await.unwrap();
            assert!(txn.get("key".into()).await.unwrap().is_none());
        }
    }
}

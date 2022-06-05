use async_trait::async_trait;
use parking_lot::Mutex;
use std::{collections::BTreeMap, sync::Arc};

use crate::error::Result;

use super::{Store, Transaction};

type StringMap = BTreeMap<String, String>;

pub struct MemoryStore {
    pub store: Arc<Mutex<StringMap>>,
}

impl Default for MemoryStore {
    fn default() -> Self {
        MemoryStore {
            store: Arc::new(Mutex::new(StringMap::new())),
        }
    }
}

#[async_trait]
impl Store for MemoryStore {
    type Trans = MemoryTransaction;
    async fn txn_begin(&self) -> Result<MemoryTransaction> {
        // remain locked for this whole transaction
        let guard = self.store.lock();
        std::mem::forget(guard);
        Ok(MemoryTransaction {
            store: self.store.clone(),
        })
    }
}

pub struct MemoryTransaction {
    store: Arc<Mutex<StringMap>>,
}

impl MemoryTransaction {
    fn get_map(&self) -> &StringMap {
        unsafe { self.store.data_ptr().as_ref().unwrap() }
    }
    fn get_map_mut(&mut self) -> &mut StringMap {
        unsafe { self.store.data_ptr().as_mut().unwrap() }
    }
}

#[async_trait]
impl Transaction for MemoryTransaction {
    async fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.get_map().get(&key).map(String::from))
    }
    async fn get_for_update(&mut self, key: String) -> Result<Option<String>> {
        self.get(key).await
    }
    async fn put(&mut self, key: String, value: String) -> Result<()> {
        self.get_map_mut().insert(key, value);
        Ok(())
    }
    async fn commit(&mut self) -> Result<()> {
        Ok(())
    }
    async fn rollback(&mut self) -> Result<()> {
        unimplemented!();
    }
}

impl Drop for MemoryTransaction {
    fn drop(&mut self) {
        unsafe { self.store.force_unlock() };
    }
}

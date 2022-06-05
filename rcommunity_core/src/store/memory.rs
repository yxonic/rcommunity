use async_trait::async_trait;
use std::sync::RwLock;
use std::{collections::BTreeMap, sync::Arc};

use crate::error::Result;

use super::{Store, Transaction};

type StringMap = Arc<RwLock<BTreeMap<String, String>>>;

#[derive(Debug, Default)]
pub struct MemoryStore {
    pub store: StringMap,
}

#[async_trait]
impl Store for MemoryStore {
    type Trans = MemoryTransaction;
    async fn txn_begin(&self) -> Result<MemoryTransaction> {
        Ok(MemoryTransaction {
            store: self.store.clone(),
        })
    }
}

pub struct MemoryTransaction {
    store: StringMap,
}

#[async_trait]
impl Transaction for MemoryTransaction {
    async fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.store.read().unwrap().get(&key).map(String::from))
    }
    async fn put(&mut self, key: String, value: String) -> Result<()> {
        self.store.write().unwrap().insert(key, value);
        Ok(())
    }
    async fn commit(&mut self) -> Result<()> {
        Ok(())
    }
    async fn rollback(&mut self) -> Result<()> {
        unimplemented!();
    }
}

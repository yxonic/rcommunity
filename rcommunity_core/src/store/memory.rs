use async_trait::async_trait;
use parking_lot::{Condvar, Mutex, MutexGuard};
use std::{collections::BTreeMap, sync::Arc};

use crate::error::Result;

use super::{Store, Transaction};

type StringMap = BTreeMap<String, String>;

#[derive(Debug, Default)]
pub struct MemoryStore {
    store: Arc<Mutex<StringMap>>,
    cur_txn_id: Arc<(Mutex<usize>, Condvar)>,
    max_txn_id: usize,
}

#[async_trait]
impl Store for MemoryStore {
    type Trans = MemoryTransaction;
    async fn txn_begin(&mut self) -> Result<MemoryTransaction> {
        self.max_txn_id += 1;
        Ok(MemoryTransaction {
            store: self.store.clone(),
            cur_txn_id: self.cur_txn_id.clone(),
            id: self.max_txn_id,
        })
    }
}

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
    async fn commit(&mut self) -> Result<()> {
        self.release_txn_lock();
        Ok(())
    }
    async fn rollback(&mut self) -> Result<()> {
        unimplemented!();
    }
}

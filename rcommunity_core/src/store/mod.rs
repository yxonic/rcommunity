pub mod memory;

use async_trait::async_trait;

use crate::error::Result;

#[async_trait]
pub trait Store {
    type Trans: Transaction;
    async fn txn_begin(&self) -> Result<Self::Trans>;
}

#[async_trait]
pub trait Transaction: Send {
    async fn get(&self, key: String) -> Result<Option<String>>;
    async fn get_for_update(&mut self, key: String) -> Result<Option<String>>;
    async fn put(&mut self, key: String, value: String) -> Result<()>;
    async fn commit(&mut self) -> Result<()>;
    async fn rollback(&mut self) -> Result<()>;
}

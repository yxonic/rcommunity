//! Traits and implementations for the backing storage layer.

pub mod format;
pub mod memory;

use async_trait::async_trait;

use crate::error::Result;

/// Abstraction for the backing storage layer. Represents a transactional API.
#[async_trait]
pub trait Store {
    /// The corresponding [`Transaction`] type.
    type Transaction: Transaction;
    /// Begins a transaction for further operations. See [`Transaction`].
    async fn begin_txn(&mut self) -> Result<Self::Transaction>;
}

/// Abstraction of all supported operations in a transaction corresponding to a [`Store`].
#[async_trait]
pub trait Transaction: Send + Sync {
    /// Get the value of a key from store.
    async fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    /// Get the value of a key from store, while blocking reads/writes from other transactions.
    /// Useful for concurrent global updates.
    async fn get_for_update(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    /// Put a key-value pair in store.
    async fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    /// Deletes the given key and its value from store.
    async fn delete(&mut self, key: &[u8]) -> Result<()>;
    /// Commit this transaction.
    async fn commit(&mut self) -> Result<()>;
    /// Rollback this transaction. Implementation of this method is not required.
    async fn rollback(&mut self) -> Result<()> {
        unimplemented!("this transaction does not support rollback");
    }
}

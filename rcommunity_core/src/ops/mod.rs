//! Traits for supporting all the internal operations.

use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::{
    error::Result,
    markers::{ItemType, ReactionType, UserType},
    store::Transaction,
};

mod before_store;
mod reaction_info;
mod unique_index;
mod enum_index;

#[cfg(test)]
mod tests;

pub use before_store::BeforeStore;
pub use enum_index::EnumIndex;
pub use reaction_info::{ReactionInfo, ReactionInfoOnce};
pub use unique_index::UniqueIndex;

#[async_trait]
pub trait Reactor {
    async fn react(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &(impl UserType + DeserializeOwned),
        item: &(impl ItemType + DeserializeOwned),
    ) -> Result<()>;
    async fn dereact<TU, TI>(txn: &mut impl Transaction, rid: &str) -> Result<()>
    where
        TU: UserType + DeserializeOwned,
        TI: ItemType + DeserializeOwned;
}

#[async_trait]
impl<T: ReactionType + DeserializeOwned> Reactor for T {
    async fn react(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &(impl UserType + DeserializeOwned),
        item: &(impl ItemType + DeserializeOwned),
    ) -> Result<()> {
        self.before_store(txn, user, item).await?;
        self.store_reaction(txn, rid, user, item).await?;
        self.store_unique_index(txn, rid, user, item).await?;
        self.store_enum_index(txn, rid, user, item).await?;
        Ok(())
    }
    async fn dereact<TU, TI>(txn: &mut impl Transaction, rid: &str) -> Result<()>
    where
        TU: UserType + DeserializeOwned,
        TI: ItemType + DeserializeOwned,
    {
        let r = T::get_reaction_by_id::<TU, TI>(txn, rid).await?;
        let user = r.user;
        let item = r.item;
        let r = r.reaction;
        r.discard_enum_index(txn, rid, &user, &item).await?;
        r.discard_unique_index(txn, rid, &user, &item).await?;
        r.discard_reaction(txn, rid, &user, &item).await?;
        Ok(())
    }
}

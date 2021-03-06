//! Traits for supporting all the internal operations.

use async_trait::async_trait;

use crate::{error::Result, store::Transaction};

use crate::markers::{ItemType, ReactionType, UserType};

mod before_store;
mod enum_index;
mod reaction_info;
mod unique_index;

pub use before_store::BeforeStore;
pub use enum_index::EnumIndex;
pub use reaction_info::ReactionInfo;
pub use unique_index::UniqueIndex;

#[async_trait]
pub trait Reactor {
    async fn react(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
    async fn dereact<TU: UserType, TI: ItemType>(
        txn: &mut impl Transaction,
        rid: &str,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType> Reactor for T {
    async fn react(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        self.before_store(txn, user, item).await?;
        self.store_reaction(txn, rid, user, item).await?;
        self.store_unique_index(txn, rid, user, item).await?;
        self.store_enum_index(txn, rid, user, item).await?;
        Ok(())
    }
    async fn dereact<TU: UserType, TI: ItemType>(
        txn: &mut impl Transaction,
        rid: &str,
    ) -> Result<()> {
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

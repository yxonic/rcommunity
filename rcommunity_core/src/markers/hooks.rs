use async_trait::async_trait;

use crate::{error::Result, store::Transaction, utils::typename};

use super::{ItemType, ReactionType, UserType};
use super::{Once, ID};

#[async_trait]
pub trait BeforeStore {
    async fn before_store(
        &self,
        txn: &mut impl Transaction,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType> BeforeStore for T {
    default async fn before_store(
        &self,
        _txn: &mut impl Transaction,
        _user: &impl UserType,
        _item: &impl ItemType,
    ) -> Result<()> {
        // by default do nothing
        Ok(())
    }
}

#[async_trait]
pub trait OnStoreReaction {
    async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType> OnStoreReaction for T {
    default async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!("{typename}:{}:{}:{rid}", user.serialize(), item.serialize());
        txn.put(key, self.serialize()).await?;
        Ok(())
    }
}

#[async_trait]
pub trait OnStoreUniqueIndex {
    async fn store_unique_index(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType> OnStoreUniqueIndex for T {
    default async fn store_unique_index(
        &self,
        _txn: &mut impl Transaction,
        _rid: &str,
        _user: &impl UserType,
        _item: &impl ItemType,
    ) -> Result<()> {
        // by default do nothing
        Ok(())
    }
}

#[async_trait]
impl<T: ReactionType + ID> OnStoreUniqueIndex for T {
    async fn store_unique_index(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!(
            "{typename}:{}:{}:{}",
            user.serialize(),
            item.serialize(),
            self.serialize()
        );
        txn.put(key, rid.into()).await?;
        Ok(())
    }
}

#[async_trait]
impl<T: ReactionType + Once> BeforeStore for T {
    async fn before_store(
        &self,
        _txn: &mut impl Transaction,
        _user: &impl UserType,
        _item: &impl ItemType,
    ) -> Result<()> {
        // TODO: dereact
        Ok(())
    }
}

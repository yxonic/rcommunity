use async_trait::async_trait;

use crate::{error::Result, store::Transaction, utils::typename};

use super::{ItemType, ReactionType, UserType};
use super::{Once, ID};

#[async_trait]
pub trait Reactor {
    async fn react(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
    async fn dereact(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
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
        Ok(())
    }
    async fn dereact(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        self.discard_reaction(txn, rid, user, item).await?;
        self.discard_unique_index(txn, rid, user, item).await?;
        Ok(())
    }
}

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
pub trait ReactionInfo {
    async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
    async fn discard_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType> ReactionInfo for T {
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
    default async fn discard_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!("{typename}:{}:{}:{rid}", user.serialize(), item.serialize());
        txn.delete(key).await?;
        Ok(())
    }
}

#[async_trait]
pub trait UniqueIndex {
    async fn store_unique_index(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
    async fn discard_unique_index(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType> UniqueIndex for T {
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
    default async fn discard_unique_index(
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
impl<T: ReactionType + Once> ReactionInfo for T {
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
        let key = format!("{typename}:{}:{}", user.serialize(), item.serialize());
        txn.put(key, rid.into()).await?;
        Ok(())
    }
    default async fn discard_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!("{typename}:{}:{}:{rid}", user.serialize(), item.serialize());
        txn.delete(key).await?;
        let key = format!("{typename}:{}:{}", user.serialize(), item.serialize());
        txn.delete(key).await?;
        Ok(())
    }
}

#[async_trait]
impl<T: ReactionType + ID> UniqueIndex for T {
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
    async fn discard_unique_index(
        &self,
        txn: &mut impl Transaction,
        _rid: &str,
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
        txn.delete(key).await?;
        Ok(())
    }
}

#[async_trait]
impl<T: ReactionType + Once> BeforeStore for T {
    async fn before_store(
        &self,
        txn: &mut impl Transaction,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!("{typename}:{}:{}", user.serialize(), item.serialize());
        let rid = txn.get(key).await?;
        if let Some(rid) = rid {
            self.dereact(txn, &rid, user, item).await?;
        }
        Ok(())
    }
}

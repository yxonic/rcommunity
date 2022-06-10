use async_trait::async_trait;

use crate::{error::Result, store::Transaction, utils::typename};

use crate::markers::{Enumerable, Once, ID};
use crate::markers::{ItemType, ReactionType, UserType};

mod before_store;

pub use crate::ops::before_store::BeforeStore;

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
        let key = format!("{typename}_{}_{}_{rid}", user.serialize(), item.serialize());
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
        let key = format!("{typename}_{}_{}_{rid}", user.serialize(), item.serialize());
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
pub trait EnumIndex {
    async fn store_enum_index(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
    async fn discard_enum_index(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType> EnumIndex for T {
    default async fn store_enum_index(
        &self,
        _txn: &mut impl Transaction,
        _rid: &str,
        _user: &impl UserType,
        _item: &impl ItemType,
    ) -> Result<()> {
        // by default do nothing
        Ok(())
    }
    default async fn discard_enum_index(
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
        self.store_enum_index(txn, rid, user, item).await?;
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
        self.discard_enum_index(txn, rid, user, item).await?;
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
        let key = format!("{typename}_{}_{}", user.serialize(), item.serialize());
        txn.put(key, format!("{}_{rid}", self.serialize())).await?;
        Ok(())
    }
    default async fn discard_reaction(
        &self,
        txn: &mut impl Transaction,
        _rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!("{typename}_{}_{}", user.serialize(), item.serialize());
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
            "{typename}_{}_{}_{}",
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
            "{typename}_{}_{}_{}",
            user.serialize(),
            item.serialize(),
            self.serialize()
        );
        txn.delete(key).await?;
        Ok(())
    }
}

#[async_trait]
impl<T: ReactionType + Enumerable> EnumIndex for T {
    async fn store_enum_index(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!(
            "{typename}_{}_{}_{}_{rid}",
            user.serialize(),
            item.serialize(),
            self.serialize()
        );
        txn.put(key, "".into()).await?;
        Ok(())
    }
    async fn discard_enum_index(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!(
            "{typename}_{}_{}_{}_{rid}",
            user.serialize(),
            item.serialize(),
            self.serialize()
        );
        txn.delete(key).await?;
        Ok(())
    }
}

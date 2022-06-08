use async_trait::async_trait;

use crate::{error::Result, store::Transaction, utils::typename};

use super::{ItemType, ReactionType, UserType, ID};

#[async_trait]
pub trait Storable {
    async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
    async fn store_unique_index(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType> Storable for T {
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
impl<T: ReactionType + ID> Storable for T {
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

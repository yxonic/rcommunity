use async_trait::async_trait;

use crate::{error::Result, store::Transaction, utils::typename};

use crate::markers::Once;
use crate::markers::{ItemType, ReactionType, UserType};

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

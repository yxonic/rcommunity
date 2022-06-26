use async_trait::async_trait;

use crate::{error::Result, store::Transaction, utils::typename};

use crate::markers::Once;
use crate::markers::{ItemType, ReactionType, UserType};

/// Ability to manage and query reaction basic information.
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

/// Default [`ReactionInfo`] implementor for all reaction types.
///
/// Under the hood, this implementor manages **reaction ID** to **user-item-reaction triplet**
/// mapping for all reaction types.
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

/// Specialized [`ReactionInfo`] implementor for reaction types that are marked as [`Once`].
///
/// Under the hood, this implementor manages **reaction ID** to **user-item pair** mapping for
/// reaction types that react at most once for each user-item pair.
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

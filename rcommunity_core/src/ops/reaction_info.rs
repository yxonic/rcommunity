use async_trait::async_trait;

use crate::error::Error;
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
    async fn get_reaction_by_id<TU, TI>(
        txn: &mut impl Transaction,
        rid: &str,
    ) -> Result<(TU, TI, Self)>
    where
        TU: UserType,
        TI: ItemType,
        Self: Sized;
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
        let key = format!("r_{typename}_{rid}");
        txn.put(
            key,
            format!(
                "{}_{}_{}",
                user.serialize(),
                item.serialize(),
                self.serialize()
            ),
        )
        .await?;
        let key = format!(
            "ui_{typename}_{}_{}_{rid}",
            user.serialize(),
            item.serialize()
        );
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
        let key = format!("r_{typename}_{rid}");
        txn.delete(key).await?;
        let key = format!(
            "ui_{typename}_{}_{}_{rid}",
            user.serialize(),
            item.serialize()
        );
        txn.delete(key).await?;
        Ok(())
    }
    default async fn get_reaction_by_id<TU: UserType, TI: ItemType>(
        txn: &mut impl Transaction,
        rid: &str,
    ) -> Result<(TU, TI, Self)> {
        let typename = typename::<T>();
        let key = format!("r_{typename}_{rid}");
        let value = txn.get(key).await?;
        if let Some(v) = value {
            let fields: Vec<&str> = v.split('_').collect();
            let user = TU::deserialize(fields[0]);
            let item = TI::deserialize(fields[1]);
            let r = T::deserialize(fields[2]);
            return Ok((user, item, r));
        }
        Err(Error::UnknownError("TODO: change to not found".into()))
    }
}

/// Specialized [`ReactionInfo`] implementor for reaction types that are marked as [`Once`].
///
/// Under the hood, this implementor manages **reaction ID** to **user-item pair** mapping for
/// reaction types that react at most once for each user-item pair.
#[async_trait]
impl<T: ReactionType + Once> ReactionInfo for T {
    async fn store_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!("r_{typename}_{rid}");
        txn.put(
            key,
            format!(
                "{}_{}_{}",
                user.serialize(),
                item.serialize(),
                self.serialize()
            ),
        )
        .await?;
        let key = format!("ui_{typename}_{}_{}", user.serialize(), item.serialize());
        txn.put(key, format!("{}_{rid}", self.serialize())).await?;
        Ok(())
    }
    async fn discard_reaction(
        &self,
        txn: &mut impl Transaction,
        rid: &str,
        user: &impl UserType,
        item: &impl ItemType,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!("r_{typename}_{rid}");
        txn.delete(key).await?;
        let key = format!("ui_{typename}_{}_{}", user.serialize(), item.serialize());
        txn.delete(key).await?;
        Ok(())
    }
}

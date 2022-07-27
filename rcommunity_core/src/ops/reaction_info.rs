use async_trait::async_trait;

use crate::error::Error;
use crate::{
    error::Result,
    store::{Key, Transaction, Value},
    utils::typename,
};

use crate::markers::Once;
use crate::markers::{ItemType, ReactionType, UserType};

pub struct Reaction<TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
{
    pub user: TU,
    pub item: TI,
    pub reaction: TR,
}

/// Ability to manage and query reaction basic information.
#[async_trait]
pub trait ReactionInfo: ReactionType {
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
    async fn get_reaction_by_id<TU: UserType, TI: ItemType>(
        txn: &mut impl Transaction,
        rid: &str,
    ) -> Result<Reaction<TU, TI, Self>>;
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
            Key::raw(key.as_bytes().to_vec()),
            Value::raw(
                format!(
                    "{}_{}_{}",
                    user.serialize(),
                    item.serialize(),
                    self.serialize()
                )
                .as_bytes()
                .to_vec(),
            ),
        )
        .await?;
        let key = format!(
            "ui_{typename}_{}_{}_{rid}",
            user.serialize(),
            item.serialize()
        );
        txn.put(
            Key::raw(key.as_bytes().to_vec()),
            Value::raw(self.serialize().as_bytes().to_vec()),
        )
        .await?;
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
        txn.delete(Key::raw(key.as_bytes().to_vec())).await?;
        let key = format!(
            "ui_{typename}_{}_{}_{rid}",
            user.serialize(),
            item.serialize()
        );
        txn.delete(Key::raw(key.as_bytes().to_vec())).await?;
        Ok(())
    }
    default async fn get_reaction_by_id<TU: UserType, TI: ItemType>(
        txn: &mut impl Transaction,
        rid: &str,
    ) -> Result<Reaction<TU, TI, T>> {
        let typename = typename::<T>();
        let key = format!("r_{typename}_{rid}");
        let value = txn.get(Key::raw(key.as_bytes().to_vec())).await?;
        if let Some(v) = value {
            let v = String::from_utf8(v.0).unwrap();
            let fields: Vec<&str> = v.split('_').collect();
            let user = TU::deserialize(fields[0]);
            let item = TI::deserialize(fields[1]);
            let reaction = T::deserialize(fields[2]);
            return Ok(Reaction {
                user,
                item,
                reaction,
            });
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
            Key::raw(key.as_bytes().to_vec()),
            Value::raw(
                format!(
                    "{}_{}_{}",
                    user.serialize(),
                    item.serialize(),
                    self.serialize()
                )
                .as_bytes()
                .to_vec(),
            ),
        )
        .await?;
        let key = format!("ui_{typename}_{}_{}", user.serialize(), item.serialize());
        txn.put(
            Key::raw(key.as_bytes().to_vec()),
            Value::raw(format!("{}_{rid}", self.serialize()).as_bytes().to_vec()),
        )
        .await?;
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
        txn.delete(Key::raw(key.as_bytes().to_vec())).await?;
        let key = format!("ui_{typename}_{}_{}", user.serialize(), item.serialize());
        txn.delete(Key::raw(key.as_bytes().to_vec())).await?;
        Ok(())
    }
}

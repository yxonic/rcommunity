use async_trait::async_trait;

use crate::Serializable;
use crate::{error::Result, store::Transaction, utils::typename};

use crate::markers::Enumerable;
use crate::markers::{ItemType, ReactionType, UserType};

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
            Serializable::serialize(user),
            Serializable::serialize(item),
            Serializable::serialize(self)
        );
        txn.put(key.as_bytes(), b"").await?;
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
            Serializable::serialize(user),
            Serializable::serialize(item),
            Serializable::serialize(self)
        );
        txn.delete(key.as_bytes()).await?;
        Ok(())
    }
}

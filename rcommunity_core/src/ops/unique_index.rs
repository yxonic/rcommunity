use async_trait::async_trait;

use crate::{
    error::Result,
    store::{Key, Transaction, Value},
    utils::typename,
};

use crate::markers::ID;
use crate::markers::{ItemType, ReactionType, UserType};

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
            "u_{typename}_{}_{}_{}",
            user.serialize(),
            item.serialize(),
            self.serialize()
        );
        txn.put(Key::raw(key), Value::raw(rid)).await?;
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
            "u_{typename}_{}_{}_{}",
            user.serialize(),
            item.serialize(),
            self.serialize()
        );
        txn.delete(Key::raw(key)).await?;
        Ok(())
    }
}

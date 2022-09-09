use async_trait::async_trait;

use crate::{error::Result, store::Transaction};

use crate::markers::{ItemType, ReactionType, UserType, ID};

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
        _txn: &mut impl Transaction,
        _rid: &str,
        _user: &impl UserType,
        _item: &impl ItemType,
    ) -> Result<()> {
        //     let typename = typename::<T>();
        //     let key = format!(
        //         "u_{typename}_{}_{}_{}",
        //         Serializable::serialize(user),
        //         Serializable::serialize(item),
        //         Serializable::serialize(self)
        //     );
        //     txn.put(key.as_bytes(), rid.as_bytes()).await?;
        //     Ok(())
        // }
        // async fn discard_unique_index(
        //     &self,
        //     txn: &mut impl Transaction,
        //     _rid: &str,
        //     user: &impl UserType,
        //     item: &impl ItemType,
        // ) -> Result<()> {
        //     let typename = typename::<T>();
        //     let key = format!(
        //         "u_{typename}_{}_{}_{}",
        //         Serializable::serialize(user),
        //         Serializable::serialize(item),
        //         Serializable::serialize(self)
        //     );
        //     txn.delete(key.as_bytes()).await?;
        //     Ok(())
        todo!()
    }
}

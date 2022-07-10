use async_trait::async_trait;

use crate::{error::Result, store::Transaction, utils::typename};

use crate::markers::Once;
use crate::markers::{ItemType, ReactionType, UserType};
use crate::ops::Reactor;

#[async_trait]
pub trait BeforeStore {
    async fn before_store<TU: UserType, TI: ItemType>(
        &self,
        txn: &mut impl Transaction,
        user: &TU,
        item: &TI,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType> BeforeStore for T {
    default async fn before_store<TU: UserType, TI: ItemType>(
        &self,
        _txn: &mut impl Transaction,
        _user: &TU,
        _item: &TI,
    ) -> Result<()> {
        // by default do nothing
        Ok(())
    }
}

#[async_trait]
impl<T: ReactionType + Once> BeforeStore for T {
    async fn before_store<TU: UserType, TI: ItemType>(
        &self,
        txn: &mut impl Transaction,
        user: &TU,
        item: &TI,
    ) -> Result<()> {
        let typename = typename::<T>();
        let key = format!("{typename}_{}_{}", user.serialize(), item.serialize());
        let rid = txn.get(key).await?;
        if let Some(rid) = rid {
            let rid = String::from(rid);
            T::dereact::<TU, TI>(txn, &rid).await?;
        }
        Ok(())
    }
}

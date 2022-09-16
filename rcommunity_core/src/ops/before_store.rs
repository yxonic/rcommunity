use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::{
    error::Result,
    markers::{ItemType, Once, ReactionType, UserType},
    store::Transaction,
};

use super::{reaction_info::ReactionInfoOnce, Reactor};

#[async_trait]
pub trait BeforeStore {
    async fn before_store<TU: UserType + DeserializeOwned, TI: ItemType + DeserializeOwned>(
        &self,
        txn: &mut impl Transaction,
        user: &TU,
        item: &TI,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType + DeserializeOwned> BeforeStore for T {
    default async fn before_store<TU, TI>(
        &self,
        _txn: &mut impl Transaction,
        _user: &TU,
        _item: &TI,
    ) -> Result<()>
    where
        TU: UserType + DeserializeOwned,
        TI: ItemType + DeserializeOwned,
    {
        // by default do nothing
        Ok(())
    }
}

#[async_trait]
impl<T: ReactionType + DeserializeOwned + Once> BeforeStore for T {
    async fn before_store<TU, TI>(
        &self,
        txn: &mut impl Transaction,
        user: &TU,
        item: &TI,
    ) -> Result<()>
    where
        TU: UserType + DeserializeOwned,
        TI: ItemType + DeserializeOwned,
    {
        let rid = T::get_rid(txn, user, item).await;
        if let Ok(r) = &rid {
            T::dereact::<TU, TI>(txn, r).await?;
        }
        Ok(())
    }
}

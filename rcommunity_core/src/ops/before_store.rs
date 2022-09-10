use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::error::Result;
use crate::markers::{ItemType, Once, ReactionType, UserType};
use crate::store::Transaction;

use super::reaction_info::ReactionInfoOnce;
use super::Reactor;

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
    default async fn before_store<
        TU: UserType + DeserializeOwned,
        TI: ItemType + DeserializeOwned,
    >(
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
impl<T: ReactionType + DeserializeOwned + Once> BeforeStore for T {
    async fn before_store<TU: UserType + DeserializeOwned, TI: ItemType + DeserializeOwned>(
        &self,
        txn: &mut impl Transaction,
        user: &TU,
        item: &TI,
    ) -> Result<()> {
        let rid = T::get_rid(txn, user, item).await;
        if let Ok(r) = &rid {
            T::dereact::<TU, TI>(txn, r).await?;
        }
        Ok(())
    }
}

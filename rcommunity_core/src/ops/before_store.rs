use async_trait::async_trait;
use serde::Deserialize;

use crate::error::{Error, Result};
use crate::markers::{ItemType, Once, ReactionType, UserType};
use crate::store::format::{to_key, TypeName};
use crate::store::Transaction;

use super::reaction_info::UserItemToReactionOnceKeyRef;
use super::Reactor;

#[async_trait]
pub trait BeforeStore {
    async fn before_store<
        TU: UserType + for<'a> Deserialize<'a>,
        TI: ItemType + for<'a> Deserialize<'a>,
    >(
        &self,
        txn: &mut impl Transaction,
        user: &TU,
        item: &TI,
    ) -> Result<()>;
}

#[async_trait]
impl<T: ReactionType + for<'a> Deserialize<'a>> BeforeStore for T {
    default async fn before_store<
        TU: UserType + for<'a> Deserialize<'a>,
        TI: ItemType + for<'a> Deserialize<'a>,
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
impl<T: ReactionType + Once + for<'a> Deserialize<'a>> BeforeStore for T {
    async fn before_store<
        TU: UserType + for<'a> Deserialize<'a>,
        TI: ItemType + for<'a> Deserialize<'a>,
    >(
        &self,
        txn: &mut impl Transaction,
        user: &TU,
        item: &TI,
    ) -> Result<()> {
        let key = UserItemToReactionOnceKeyRef {
            reaction_type: TypeName::<T>::new(),
            user,
            item,
        };
        let rid = txn
            .get(&to_key(&key).map_err(Error::SerializationError)?)
            .await?;
        if let Some(rid) = rid {
            let rid = String::from_utf8(rid).unwrap();
            T::dereact::<TU, TI>(txn, &rid).await?;
        }
        Ok(())
    }
}

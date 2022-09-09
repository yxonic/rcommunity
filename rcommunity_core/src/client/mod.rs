//! Type-safe API for accessing community store and operations.

use std::marker::PhantomData;

use serde::Deserialize;

use crate::{
    error::Result,
    markers::{ItemType, ReactionType, UserType},
    ops::Reactor,
    store::{Store, Transaction},
};

#[derive(Debug)]
pub struct UserItemUnboundedReactionClient<
    'store,
    TS: Store,
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
> {
    store: &'store mut TS,
    user: TU,
    item: TI,
    reaction_type: PhantomData<TR>,
}

impl<
        'store,
        TS: Store,
        TU: UserType + for<'a> Deserialize<'a>,
        TI: ItemType + for<'a> Deserialize<'a>,
        TR: ReactionType + for<'a> Deserialize<'a>,
    > UserItemUnboundedReactionClient<'store, TS, TU, TI, TR>
{
    /// Create a new reaction.
    ///
    /// # Errors
    /// Will return error when internal store failed.
    pub async fn react(&mut self, reaction: impl Into<TR>) -> Result<String> {
        let r: TR = reaction.into();
        let mut txn = self.store.begin_txn().await?;
        let rid = uuid::Uuid::new_v4().to_string(); // TODO: keep Uuid type
        r.react(&mut txn, &rid, &self.user, &self.item).await?;
        txn.commit().await?;
        Ok(rid)
    }
}

use async_trait::async_trait;

use crate::{
    error::Error, Enumerable, ItemType, Multiple, Numerical, Once, ReactionType, UserType, WithData,
};

#[async_trait]
pub trait Backend<'backend>: Sync {
    async fn query_data<T, TD>(&self, key: &T) -> Result<TD, Error>
    where
        T: WithData<Item = TD> + Sync,
    {
        drop(key);
        Err(Error::NotImplemented)
    }

    async fn give_reaction<TU, TI, TR>(
        &self,
        user: &TU,
        item: &TI,
        reaction: &TR,
    ) -> Result<(), Error>
    where
        TU: UserType,
        TI: ItemType,
        TR: ReactionType + Once,
    {
        Ok(())
    }

    async fn push_reaction<TU, TI, TR>(
        &self,
        user: &TU,
        item: &TI,
        reaction: &TR,
    ) -> Result<(), Error>
    where
        TU: UserType,
        TI: ItemType,
        TR: ReactionType + Multiple,
    {
        Ok(())
    }

    async fn query_reaction<TU, TI, TR>(&self, user: &TU, item: &TI) -> Result<TR, Error>
    where
        TU: UserType,
        TI: ItemType,
        TR: ReactionType + Once,
    {
        drop(user);
        drop(item);
        Err(Error::NotImplemented)
    }

    async fn query_reactions<TU, TI, TR>(&self, user: &TU, item: &TI) -> Result<TR, Error>
    where
        TU: UserType,
        TI: ItemType,
        TR: ReactionType + Multiple,
    {
        drop(user);
        drop(item);
        Err(Error::NotImplemented)
    }

    async fn query_given_count<TU, TR>(&self, user: &TU) -> Result<usize, Error>
    where
        TU: UserType + Sync,
        TR: ReactionType + Sync,
    {
        drop(user);
        Ok(0)
    }

    async fn query_given_count_for_reaction<TU, TR>(
        &self,
        user: &TU,
        reaction: &TR,
    ) -> Result<usize, Error>
    where
        TU: UserType + Sync,
        TR: ReactionType + Enumerable + Sync,
    {
        drop(user);
        drop(reaction);
        Ok(0)
    }

    async fn query_given_sum<TU, TR, TN>(&self, user: &TU) -> Result<TN, Error>
    where
        TU: UserType + Sync,
        TR: ReactionType + Numerical<Item = TN> + Sync,
    {
        drop(user);
        Err(Error::NotImplemented)
    }

    async fn query_given_mean<TU, TR, TN>(&self, user: &TU) -> Result<f64, Error>
    where
        TU: UserType + Sync,
        TR: ReactionType + Numerical<Item = TN> + Sync,
    {
        drop(user);
        Err(Error::NotImplemented)
    }

    async fn query_received_count<TI, TR>(&self, item: &TI) -> Result<usize, Error>
    where
        TI: ItemType + Sync,
        TR: ReactionType + Sync,
    {
        drop(item);
        Ok(0)
    }

    async fn query_received_count_for_reaction<TI, TR>(
        &self,
        item: &TI,
        reaction: &TR,
    ) -> Result<usize, Error>
    where
        TI: ItemType + Sync,
        TR: ReactionType + Enumerable + Sync,
    {
        drop(item);
        drop(reaction);
        Ok(0)
    }

    async fn query_received_sum<TI, TR, TN>(&self, item: &TI) -> Result<TN, Error>
    where
        TI: ItemType + Sync,
        TR: ReactionType + Numerical<Item = TN> + Sync,
    {
        drop(item);
        Err(Error::NotImplemented)
    }

    async fn query_received_mean<TI, TR, TN>(&self, item: &TI) -> Result<f64, Error>
    where
        TI: ItemType + Sync,
        TR: ReactionType + Numerical<Item = TN> + Sync,
    {
        drop(item);
        Err(Error::NotImplemented)
    }
}

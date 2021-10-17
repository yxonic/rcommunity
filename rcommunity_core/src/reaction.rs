use std::marker::PhantomData;

use crate::{
    backend::Backend, error::Error, Enumerable, ItemType, Numerical, Once, ReactionType, UserType,
    WithData,
};

pub struct ReactionsQuery<'backend, TB: Backend<'backend>, TR: ReactionType> {
    pub(crate) backend: &'backend TB,
    reaction_type: PhantomData<TR>,
}

pub struct ReactionQuery<'backend, TB: Backend<'backend>, TR: ReactionType> {
    pub(crate) reaction_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) reaction_type: PhantomData<TR>,
}

pub struct UserReactionsQuery<'backend, TB: Backend<'backend>, TU: UserType, TR: ReactionType> {
    pub(crate) user_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
    pub(crate) reaction_type: PhantomData<TR>,
}

pub struct UserReactionQuery<'backend, TB: Backend<'backend>, TU: UserType, TR: ReactionType> {
    pub(crate) user_id: String,
    pub(crate) reaction: TR,
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
}

pub struct ItemReactionsQuery<'backend, TB: Backend<'backend>, TI: ItemType, TR: ReactionType> {
    pub(crate) item_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
    pub(crate) reaction_type: PhantomData<TR>,
}

pub struct ItemReactionQuery<'backend, TB: Backend<'backend>, TI: ItemType, TR: ReactionType> {
    pub(crate) item_id: String,
    pub(crate) reaction: TR,
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
}

pub struct UserItemReactionsQuery<
    'backend,
    TB: Backend<'backend>,
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
> {
    pub(crate) user_id: String,
    pub(crate) item_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
    pub(crate) item_type: PhantomData<TI>,
    pub(crate) reaction_type: PhantomData<TR>,
}

pub struct UserItemReactionQuery<
    'backend,
    TB: Backend<'backend>,
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
> {
    pub(crate) user_id: String,
    pub(crate) item_id: String,
    pub(crate) reaction: TR,
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
    pub(crate) item_type: PhantomData<TI>,
}

impl<'backend, TB: Backend<'backend>, TR> ReactionsQuery<'backend, TB, TR>
where
    TR: ReactionType,
{
    fn get(&self, id: impl Into<String>) -> ReactionQuery<'backend, TB, TR> {
        ReactionQuery {
            reaction_id: id.into(),
            backend: self.backend,
            reaction_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TR> UserReactionsQuery<'backend, TB, TU, TR>
where
    TU: UserType,
    TR: ReactionType,
{
    async fn count(&self) -> Result<usize, Error> {
        todo!("return given reaction count")
    }
}

impl<'backend, TB: Backend<'backend>, TU, TR, TN> UserReactionsQuery<'backend, TB, TU, TR>
where
    TU: UserType,
    TR: ReactionType + Numerical<Item = TN>,
{
    async fn sum(&self) -> Result<TN, Error> {
        todo!("return given sum for numerical reactions")
    }
    async fn mean(&self) -> Result<f64, Error> {
        todo!("return given mean for numerical reactions")
    }
}

impl<'backend, TB: Backend<'backend>, TI, TR> ItemReactionsQuery<'backend, TB, TI, TR>
where
    TI: ItemType,
    TR: ReactionType,
{
    async fn count(&self) -> Result<usize, Error> {
        todo!("return received reaction count")
    }
}

impl<'backend, TB: Backend<'backend>, TI, TR, TN> ItemReactionsQuery<'backend, TB, TI, TR>
where
    TI: ItemType,
    TR: ReactionType + Numerical<Item = TN>,
{
    async fn sum(&self) -> Result<TN, Error> {
        todo!("return received sum for numerical reactions")
    }
    async fn mean(&self) -> Result<f64, Error> {
        todo!("return received mean for numerical reactions")
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR> UserItemReactionsQuery<'backend, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + Once,
{
    async fn get(&self) -> Result<TR, Error> {
        todo!("return the unique reaction")
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR, TD>
    UserItemReactionsQuery<'backend, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + Once + WithData<Item = TD>,
{
    async fn get_with_data(&self) -> Result<(TR, TD), Error> {
        todo!("return the unique reaction with data")
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR> UserItemReactionsQuery<'backend, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + Enumerable,
{
    async fn get_reaction(
        &self,
        reaction: impl Into<TR>,
    ) -> UserItemReactionQuery<'backend, TB, TU, TI, TR> {
        UserItemReactionQuery {
            user_id: self.user_id.to_owned(),
            item_id: self.item_id.to_owned(),
            reaction: reaction.into(),
            backend: self.backend,
            user_type: PhantomData,
            item_type: PhantomData,
        }
    }
}

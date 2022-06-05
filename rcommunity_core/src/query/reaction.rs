use std::marker::PhantomData;

use crate::{
    backend::Backend, error::Error, Enumerable, ItemType, Multiple, Numerical, Once, ReactionType,
    UserType, WithData,
};

#[derive(Debug)]
pub struct ReactionsQuery<'backend, TB: Backend<'backend>, TR: ReactionType> {
    backend: &'backend TB,
    reaction_type: PhantomData<TR>,
}

#[derive(Debug)]
pub struct ReactionQuery<'backend, TB: Backend<'backend>, TR: ReactionType> {
    pub(crate) reaction: TR,
    backend: &'backend TB,
}

#[derive(Debug)]
pub struct UserReactionsQuery<'backend, TB: Backend<'backend>, TU: UserType, TR: ReactionType> {
    pub(crate) user: TU,
    backend: &'backend TB,
    pub(crate) reaction_type: PhantomData<TR>,
}

#[derive(Debug)]
pub struct UserReactionQuery<'backend, TB: Backend<'backend>, TU: UserType, TR: ReactionType> {
    pub(crate) user: TU,
    pub(crate) reaction: TR,
    backend: &'backend TB,
}

#[derive(Debug)]
pub struct ItemReactionsQuery<'backend, TB: Backend<'backend>, TI: ItemType, TR: ReactionType> {
    pub(crate) item: TI,
    backend: &'backend TB,
    pub(crate) reaction_type: PhantomData<TR>,
}

#[derive(Debug)]
pub struct ItemReactionQuery<'backend, TB: Backend<'backend>, TI: ItemType, TR: ReactionType> {
    pub(crate) item: TI,
    pub(crate) reaction: TR,
    backend: &'backend TB,
}

#[derive(Debug)]
pub struct UserItemReactionsQuery<
    'backend,
    TB: Backend<'backend>,
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
> {
    pub(crate) user: TU,
    pub(crate) item: TI,
    backend: &'backend TB,
    pub(crate) reaction_type: PhantomData<TR>,
}

#[derive(Debug)]
pub struct UserItemReactionQuery<
    'backend,
    TB: Backend<'backend>,
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
> {
    pub(crate) user: TU,
    pub(crate) item: TI,
    pub(crate) reaction: TR,
    backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
    pub(crate) item_type: PhantomData<TI>,
}

impl<'backend, TB: Backend<'backend>, TR> ReactionsQuery<'backend, TB, TR>
where
    TR: ReactionType,
{
    pub fn get(&self, reaction: impl Into<TR>) -> ReactionQuery<'backend, TB, TR> {
        ReactionQuery {
            reaction: reaction.into(),
            backend: self.backend,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TR> UserReactionsQuery<'backend, TB, TU, TR>
where
    TU: UserType,
    TR: ReactionType,
{
    pub fn new(
        backend: &'backend TB,
        user: impl Into<TU>,
    ) -> UserReactionsQuery<'backend, TB, TU, TR> {
        UserReactionsQuery {
            user: user.into(),
            backend,
            reaction_type: PhantomData,
        }
    }
    pub async fn count(&self) -> Result<usize, Error> {
        self.backend.query_given_count::<_, TR>(&self.user).await
    }
}

impl<'backend, TB: Backend<'backend>, TU, TR, TN> UserReactionsQuery<'backend, TB, TU, TR>
where
    TU: UserType,
    TR: ReactionType + Numerical<Item = TN>,
{
    pub async fn sum(&self) -> Result<TN, Error> {
        todo!("return given sum for numerical reactions")
    }
    pub async fn mean(&self) -> Result<f64, Error> {
        todo!("return given mean for numerical reactions")
    }
}

impl<'backend, TB: Backend<'backend>, TI, TR> ItemReactionsQuery<'backend, TB, TI, TR>
where
    TI: ItemType,
    TR: ReactionType,
{
    pub fn new(
        backend: &'backend TB,
        item: impl Into<TI>,
    ) -> ItemReactionsQuery<'backend, TB, TI, TR> {
        ItemReactionsQuery {
            item: item.into(),
            backend,
            reaction_type: PhantomData,
        }
    }
    pub async fn count(&self) -> Result<usize, Error> {
        self.backend.query_received_count::<_, TR>(&self.item).await
    }
}

impl<'backend, TB: Backend<'backend>, TI, TR, TN> ItemReactionsQuery<'backend, TB, TI, TR>
where
    TI: ItemType,
    TR: ReactionType + Numerical<Item = TN>,
{
    pub async fn sum(&self) -> Result<TN, Error> {
        self.backend
            .query_received_sum::<_, TR, _>(&self.item)
            .await
    }
    pub async fn mean(&self) -> Result<f64, Error> {
        todo!("return received mean for numerical reactions")
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR> UserItemReactionsQuery<'backend, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
{
    pub fn new(
        backend: &'backend TB,
        user: impl Into<TU>,
        item: impl Into<TI>,
    ) -> UserItemReactionsQuery<'backend, TB, TU, TI, TR> {
        UserItemReactionsQuery {
            user: user.into(),
            item: item.into(),
            backend,
            reaction_type: PhantomData,
        }
    }
    pub fn create(&self, reaction: impl Into<TR>) -> ReactionBuilder<'backend, '_, TB, TU, TI, TR> {
        ReactionBuilder::new(self.backend, &self.user, &self.item, reaction.into())
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR> UserItemReactionsQuery<'backend, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + Once,
{
    pub async fn get(&self) -> Result<TR, Error> {
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
    pub async fn get_with_data(&self) -> Result<(TR, TD), Error> {
        todo!("return the unique reaction with data")
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR> UserItemReactionsQuery<'backend, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + Enumerable,
{
    pub async fn get_reaction(
        &self,
        reaction: impl Into<TR>,
    ) -> UserItemReactionQuery<'backend, TB, TU, TI, TR> {
        UserItemReactionQuery {
            user: self.user.to_owned(),
            item: self.item.to_owned(),
            reaction: reaction.into(),
            backend: self.backend,
            user_type: PhantomData,
            item_type: PhantomData,
        }
    }
}

pub struct ReactionBuilder<
    'backend,
    'a,
    TB: Backend<'backend>,
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
> {
    backend: &'backend TB,
    user: &'a TU,
    item: &'a TI,
    reaction: TR,
}

impl<'backend, 'a, TB: Backend<'backend>, TU, TI, TR> ReactionBuilder<'backend, 'a, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType,
{
    pub fn new(
        backend: &'backend TB,
        user: &'a TU,
        item: &'a TI,
        reaction: TR,
    ) -> ReactionBuilder<'backend, 'a, TB, TU, TI, TR> {
        ReactionBuilder {
            backend,
            user,
            item,
            reaction,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR, TD> ReactionBuilder<'backend, '_, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + WithData<Item = TD>,
{
    pub fn with_data(self, data: impl Into<TD>) -> Self {
        self
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR, TN> ReactionBuilder<'backend, '_, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + Numerical<Item = TN>,
{
    pub fn as_numerical(self) -> Self {
        self
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR> ReactionBuilder<'backend, '_, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + Enumerable,
{
    pub fn as_enumerable(self) -> Self {
        self
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR> ReactionBuilder<'backend, '_, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + Once,
{
    pub async fn react(self) {}
}

impl<'backend, TB: Backend<'backend>, TU, TI, TR> ReactionBuilder<'backend, '_, TB, TU, TI, TR>
where
    TU: UserType,
    TI: ItemType,
    TR: ReactionType + Multiple,
{
    pub async fn push(self) {}
}

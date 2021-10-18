use std::marker::PhantomData;

use crate::{
    backend::Backend,
    error::Error,
    query::{ItemReactionsQuery, UserReactionsQuery},
    ItemType, ReactionType, UserType, WithData,
};

use super::UserItemReactionsQuery;

#[derive(Debug)]
pub struct ItemsQuery<'backend, TB: Backend<'backend>, TI: ItemType> {
    backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
}

#[derive(Debug)]
pub struct ItemQuery<'backend, TB: Backend<'backend>, TI: ItemType> {
    pub(crate) item: TI,
    backend: &'backend TB,
}

#[derive(Debug)]
pub struct UserItemsQuery<'backend, TB: Backend<'backend>, TU: UserType, TI: ItemType> {
    pub(crate) user: TU,
    backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
}

#[derive(Debug)]
pub struct UserItemQuery<'backend, TB: Backend<'backend>, TU: UserType, TI: ItemType> {
    pub(crate) user: TU,
    pub(crate) item: TI,
    backend: &'backend TB,
}

impl<'backend, TB: Backend<'backend>, TI> ItemsQuery<'backend, TB, TI>
where
    TI: ItemType,
{
    pub fn get(&self, item: impl Into<TI>) -> ItemQuery<'backend, TB, TI> {
        ItemQuery {
            item: item.into(),
            backend: self.backend,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TI> ItemQuery<'backend, TB, TI>
where
    TI: ItemType,
{
    pub fn reaction<TR: ReactionType>(&self) -> ItemReactionsQuery<'backend, TB, TI, TR> {
        ItemReactionsQuery::new(self.backend, self.item.to_owned())
    }
}

impl<'backend, TB: Backend<'backend>, TI, TD> ItemQuery<'backend, TB, TI>
where
    TI: ItemType + WithData<Item = TD>,
{
    pub async fn data(&self) -> Result<(TI, TD), Error> {
        todo!("query user with data from backend result");
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItemsQuery<'backend, TB, TU, TI>
where
    TU: UserType,
    TI: ItemType,
{
    pub fn new(backend: &'backend TB, user: impl Into<TU>) -> UserItemsQuery<'backend, TB, TU, TI> {
        UserItemsQuery {
            user: user.into(),
            backend,
            item_type: PhantomData,
        }
    }
    pub fn get(&self, item: impl Into<TI>) -> UserItemQuery<'backend, TB, TU, TI> {
        UserItemQuery {
            user: self.user.to_owned(),
            item: item.into(),
            backend: self.backend,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItemQuery<'backend, TB, TU, TI>
where
    TU: UserType,
    TI: ItemType,
{
    pub fn reaction<TR: ReactionType>(&self) -> UserItemReactionsQuery<'backend, TB, TU, TI, TR> {
        UserItemReactionsQuery::new(self.backend, self.user.to_owned(), self.item.to_owned())
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItemsQuery<'backend, TB, TU, TI>
where
    TU: UserType,
    TI: ItemType + ReactionType,
{
    pub fn given(self) -> UserReactionsQuery<'backend, TB, TU, TI> {
        UserReactionsQuery::new(self.backend, self.user.to_owned())
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItemsQuery<'backend, TB, TU, TI>
where
    TU: UserType + ItemType,
    TI: ItemType + ReactionType,
{
    pub fn received(self) -> ItemReactionsQuery<'backend, TB, TU, TI> {
        ItemReactionsQuery::new(self.backend, self.user.to_owned())
    }
}

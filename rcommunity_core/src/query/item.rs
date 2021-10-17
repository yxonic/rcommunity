use std::marker::PhantomData;

use crate::{
    backend::Backend,
    error::Error,
    query::{ItemReactionsQuery, UserReactionsQuery},
    ItemType, ReactionType, UserType, WithData,
};

pub struct ItemsQuery<'backend, TB: Backend<'backend>, TI: ItemType> {
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
}

pub struct ItemQuery<'backend, TB: Backend<'backend>, TI: ItemType> {
    pub(crate) item_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
}

pub struct ItemUsersQuery<'backend, TB: Backend<'backend>, TI: ItemType, TU: UserType> {
    pub(crate) item_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
    pub(crate) user_type: PhantomData<TU>,
}

pub struct ItemUserQuery<'backend, TB: Backend<'backend>, TI: ItemType, TU: UserType> {
    pub(crate) item_id: String,
    pub(crate) user_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
    pub(crate) user_type: PhantomData<TU>,
}

pub struct UserItemsQuery<'backend, TB: Backend<'backend>, TU: UserType, TI: ItemType> {
    pub(crate) user_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
    pub(crate) item_type: PhantomData<TI>,
}

pub struct UserItemQuery<'backend, TB: Backend<'backend>, TU: UserType, TI: ItemType> {
    pub(crate) user_id: String,
    pub(crate) item_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
    pub(crate) item_type: PhantomData<TI>,
}

impl<'backend, TB: Backend<'backend>, TI, TU> ItemUsersQuery<'backend, TB, TI, TU>
where
    TI: ItemType,
    TU: UserType,
{
    fn get(&self, id: impl Into<String>) -> ItemUserQuery<'backend, TB, TI, TU> {
        ItemUserQuery {
            user_id: id.into(),
            item_id: self.item_id.to_owned(),
            backend: self.backend,
            user_type: PhantomData,
            item_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TI> ItemsQuery<'backend, TB, TI>
where
    TI: ItemType,
{
    fn get(&self, id: impl Into<String>) -> ItemQuery<'backend, TB, TI> {
        ItemQuery {
            item_id: id.into(),
            backend: self.backend,
            item_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TI, TD> ItemQuery<'backend, TB, TI>
where
    TI: ItemType + WithData<Item = TD>,
{
    async fn data(&self) -> Result<(TI, TD), Error> {
        self.backend.query().await;
        todo!("query user with data from backend result");
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItemsQuery<'backend, TB, TU, TI>
where
    TU: UserType,
    TI: ItemType,
{
    fn get(&self, id: impl Into<String>) -> UserItemQuery<'backend, TB, TU, TI> {
        UserItemQuery {
            user_id: self.user_id.to_owned(),
            item_id: id.into(),
            backend: self.backend,
            user_type: PhantomData,
            item_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItemsQuery<'backend, TB, TU, TI>
where
    TU: UserType,
    TI: ItemType + ReactionType,
{
    fn given(self, id: impl Into<String>) -> UserReactionsQuery<'backend, TB, TU, TI> {
        UserReactionsQuery {
            user_id: self.user_id,
            backend: self.backend,
            user_type: PhantomData,
            reaction_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItemsQuery<'backend, TB, TU, TI>
where
    TU: UserType + ItemType,
    TI: ItemType + ReactionType,
{
    fn received(self, id: impl Into<String>) -> ItemReactionsQuery<'backend, TB, TU, TI> {
        ItemReactionsQuery {
            item_id: self.user_id,
            backend: self.backend,
            item_type: PhantomData,
            reaction_type: PhantomData,
        }
    }
}

use std::marker::PhantomData;

use crate::{
    backend::Backend,
    error::Error,
    reaction::{ItemReactions, UserReactions},
    ItemType, ReactionType, UserType, WithData,
};

pub struct Items<'backend, TB: Backend<'backend>, TI: ItemType> {
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
}

pub struct Item<'backend, TB: Backend<'backend>, TI: ItemType> {
    pub(crate) item_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
}

pub struct ItemUsers<'backend, TB: Backend<'backend>, TI: ItemType, TU: UserType> {
    pub(crate) item_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
    pub(crate) user_type: PhantomData<TU>,
}

pub struct ItemUser<'backend, TB: Backend<'backend>, TI: ItemType, TU: UserType> {
    pub(crate) item_id: String,
    pub(crate) user_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) item_type: PhantomData<TI>,
    pub(crate) user_type: PhantomData<TU>,
}

pub struct UserItems<'backend, TB: Backend<'backend>, TU: UserType, TI: ItemType> {
    pub(crate) user_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
    pub(crate) item_type: PhantomData<TI>,
}

pub struct UserItem<'backend, TB: Backend<'backend>, TU: UserType, TI: ItemType> {
    pub(crate) user_id: String,
    pub(crate) item_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
    pub(crate) item_type: PhantomData<TI>,
}

impl<'backend, TB: Backend<'backend>, TI, TU> ItemUsers<'backend, TB, TI, TU>
where
    TI: ItemType,
    TU: UserType,
{
    fn get(&self, id: impl Into<String>) -> ItemUser<'backend, TB, TI, TU> {
        ItemUser {
            user_id: id.into(),
            item_id: self.item_id.to_owned(),
            backend: self.backend,
            user_type: PhantomData,
            item_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TI> Items<'backend, TB, TI>
where
    TI: ItemType,
{
    fn get(&self, id: impl Into<String>) -> Item<'backend, TB, TI> {
        Item {
            item_id: id.into(),
            backend: self.backend,
            item_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TI, TD> Item<'backend, TB, TI>
where
    TI: ItemType + WithData<Item = TD>,
{
    async fn data(&self) -> Result<(TI, TD), Error> {
        self.backend.query().await;
        todo!("query user with data from backend result");
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItems<'backend, TB, TU, TI>
where
    TU: UserType,
    TI: ItemType,
{
    fn get(&self, id: impl Into<String>) -> UserItem<'backend, TB, TU, TI> {
        UserItem {
            user_id: self.user_id.to_owned(),
            item_id: id.into(),
            backend: self.backend,
            user_type: PhantomData,
            item_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItems<'backend, TB, TU, TI>
where
    TU: UserType,
    TI: ItemType + ReactionType,
{
    fn given(self, id: impl Into<String>) -> UserReactions<'backend, TB, TU, TI> {
        UserReactions {
            user_id: self.user_id,
            backend: self.backend,
            user_type: PhantomData,
            reaction_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TI> UserItems<'backend, TB, TU, TI>
where
    TU: UserType + ItemType,
    TI: ItemType + ReactionType,
{
    fn received(self, id: impl Into<String>) -> ItemReactions<'backend, TB, TU, TI> {
        ItemReactions {
            item_id: self.user_id,
            backend: self.backend,
            item_type: PhantomData,
            reaction_type: PhantomData,
        }
    }
}

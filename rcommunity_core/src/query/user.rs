use std::marker::PhantomData;

use crate::{backend::Backend, error::Error, query::UserItemsQuery, ItemType, UserType, WithData};

#[derive(Debug)]
pub struct UsersQuery<'backend, TB: Backend<'backend>, TU: UserType> {
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
}

#[derive(Debug)]
pub struct UserQuery<'backend, TB: Backend<'backend>, TU: UserType> {
    pub(crate) user: TU,
    pub(crate) backend: &'backend TB,
}

impl<'backend, TB: Backend<'backend>, TU> UsersQuery<'backend, TB, TU>
where
    TU: UserType,
{
    pub fn new(backend: &'backend TB) -> UsersQuery<'backend, TB, TU> {
        UsersQuery {
            backend,
            user_type: PhantomData,
        }
    }
    pub fn get(&self, user: impl Into<TU>) -> UserQuery<'backend, TB, TU> {
        UserQuery {
            user: user.into(),
            backend: self.backend,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TD> UserQuery<'backend, TB, TU>
where
    TU: UserType + WithData<Item = TD>,
{
    pub async fn data(&self) -> Result<(TU, TD), Error> {
        todo!("query user with data from backend result");
    }
}

impl<'backend, TB: Backend<'backend>, TU> UserQuery<'backend, TB, TU>
where
    TU: UserType,
{
    pub fn item<TI: ItemType>(&self) -> UserItemsQuery<'backend, TB, TU, TI> {
        UserItemsQuery::new(self.backend, self.user.to_owned())
    }
}

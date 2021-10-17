use std::marker::PhantomData;

use crate::{backend::Backend, error::Error, UserType, WithData};

pub struct Users<'backend, TB: Backend<'backend>, TU: UserType> {
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
}

pub struct User<'backend, TB: Backend<'backend>, TU: UserType> {
    pub(crate) user_id: String,
    pub(crate) backend: &'backend TB,
    pub(crate) user_type: PhantomData<TU>,
}

impl<'backend, TB: Backend<'backend>, TU> Users<'backend, TB, TU>
where
    TU: UserType,
{
    fn get(&self, id: impl Into<String>) -> User<'backend, TB, TU> {
        User {
            user_id: id.into(),
            backend: self.backend,
            user_type: PhantomData,
        }
    }
}

impl<'backend, TB: Backend<'backend>, TU, TD> User<'backend, TB, TU>
where
    TU: UserType + WithData<Item = TD>,
{
    async fn data(&self) -> Result<(TU, TD), Error> {
        self.backend.query().await;
        todo!("query user with data from backend result");
    }
}

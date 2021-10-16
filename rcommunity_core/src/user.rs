use crate::{backend::Backend, ItemType, UserType};

pub struct Users<'backend, TB: Backend<'backend>> {
    backend: &'backend TB,
}

pub struct User<'backend, TB: Backend<'backend>, TU: UserType> {
    user: TU,
    backend: &'backend TB,
}

pub struct ItemUsers<'backend, TB: Backend<'backend>, TI: ItemType> {
    item: TI,
    backend: &'backend TB,
}

pub struct ItemUser<'backend, TB: Backend<'backend>, TI: ItemType, TU: UserType> {
    item: TI,
    user: TU,
    backend: &'backend TB,
}

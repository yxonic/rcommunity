use crate::{backend::Backend, ItemType, UserType};

pub struct Items<'backend, TB: Backend<'backend>> {
    backend: &'backend TB,
}

pub struct Item<'backend, TB: Backend<'backend>, TI: ItemType> {
    item: TI,
    backend: &'backend TB,
}

pub struct UserItems<'backend, TB: Backend<'backend>, TU: UserType> {
    user: TU,
    backend: &'backend TB,
}

pub struct UserItem<'backend, TB: Backend<'backend>, TU: UserType, TI: ItemType> {
    user: TU,
    item: TI,
    backend: &'backend TB,
}

use crate::{ItemType, ReactionType, UserType, backend::Backend};

pub struct Reactions<'backend, TB: Backend<'backend>> {
    backend: &'backend TB,
}

pub struct Reaction<'backend, TB: Backend<'backend>, TR: ReactionType> {
    reaction: TR,
    backend: &'backend TB,
}

pub struct UserReactions<'backend, TB: Backend<'backend>, TU: UserType> {
    user: TU,
    backend: &'backend TB,
}

pub struct UserReaction<'backend, TB: Backend<'backend>, TU: UserType, TR: ReactionType> {
    user: TU,
    reaction: TR,
    backend: &'backend TB,
}

pub struct ItemReactions<'backend, TB: Backend<'backend>, TI: ItemType> {
    item: TI,
    backend: &'backend TB,
}

pub struct ItemReaction<'backend, TB: Backend<'backend>, TI: ItemType, TR: ReactionType> {
    item: TI,
    reaction: TR,
    backend: &'backend TB,
}

pub struct UserItemReactions<'backend, TB: Backend<'backend>, TU: UserType, TI: ItemType> {
    user: TU,
    item: TI,
    backend: &'backend TB,
}

pub struct UserItemReaction<'backend, TB: Backend<'backend>, TU: UserType, TI: ItemType, TR: ReactionType> {
    user: TU,
    item: TI,
    reaction: TR,
    backend: &'backend TB,
}

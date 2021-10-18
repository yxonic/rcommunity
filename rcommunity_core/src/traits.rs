use std::ops::Add;

pub trait Unique {}

pub trait UserType: Clone + Sync + Unique {}
pub trait ItemType: Clone + Sync + Unique {}
pub trait ReactionType: Clone + Sync {}

// user type can't be reaction type
impl<T> !UserType for T where T: ReactionType {}

pub trait WithData: Unique + ReactionType {
    type Item;
}
pub trait Numerical: ReactionType {
    type Item: Add<Output = Self::Item>;
}
pub trait Enumerable: ReactionType {
    const IS_ENUMERABLE: bool = true;
}

pub trait Once: ReactionType {
    const IS_ONCE: bool = true;
}
pub trait Multiple: ReactionType {}
// reaction is either once or multiple
impl<T> !Once for T where T: Multiple {}

pub trait Unique {}

pub trait UserType: Unique {}

pub trait ItemType: Unique {}

pub trait ReactionType {}

pub trait WithData: Unique {
    type Item;
}
pub trait Numerical {
    type Item;
}
pub trait Enumerable {}
pub trait Once {}

//! Traits to mark struct as basic community types.

use crate::utils::typename;

pub trait UserType: ID + Serializable + Clone + Sync {}
pub trait ItemType: ID + Serializable + Clone + Sync {}
pub trait ReactionType: Serializable + Clone + Sync {}

pub trait Serializable {
    fn serialize(&self) -> String;
}

pub trait ID {
    fn id(&self) -> &str;
}

impl<T: ID> Serializable for T {
    fn serialize(&self) -> String {
        let typename = typename::<T>();
        format!("{typename}:{}", self.id())
    }
}

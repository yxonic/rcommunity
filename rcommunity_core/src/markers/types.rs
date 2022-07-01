//! Traits to mark data as basic community types.

use crate::utils::typename;

pub trait UserType: ID + Serializable + Clone + Send + Sync {}
pub trait ItemType: ID + Serializable + Clone + Send + Sync {}
pub trait ReactionType: Serializable + Clone + Send + Sync {}

pub trait Serializable {
    fn serialize(&self) -> String;
    fn deserialize(data: &str) -> Self;
}

pub trait ID {
    fn id(&self) -> &str;
    fn from(id: &str) -> Self;
}

impl<T: ID> Serializable for T {
    fn serialize(&self) -> String {
        let typename = typename::<T>();
        format!("{typename}:{}", self.id())
    }
    fn deserialize(id: &str) -> Self {
        T::from(id)
    }
}

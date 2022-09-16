//! Traits to mark data as basic community types.

use super::ID;
use serde::Serialize;

pub trait UserType: ID + Serialize + Clone + Send + Sync {}
pub trait ItemType: ID + Serialize + Clone + Send + Sync {}
pub trait ReactionType: Serialize + Clone + Send + Sync {}

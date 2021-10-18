#![feature(associated_type_defaults)]
#![feature(negative_impls)]

pub mod error;
pub mod traits;

pub mod query;

pub mod backend;

// re-export all traits
pub use traits::*;

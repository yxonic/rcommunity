// clippy settings
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// toggle unstable features
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod error;
pub mod traits;

pub mod query;

pub mod backend;

// re-export all traits
pub use traits::*;

pub mod client;
pub mod markers;
pub mod store;

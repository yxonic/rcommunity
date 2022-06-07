//! This crate contains core facilities for [rcommunity].
//!
//! [rcommunity]: ../rcommunity/index.html

// Global clippy settings.
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// Use the unstable specialization feature for marker dependent add/remove logic of reations.
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod client;
pub mod error;
pub mod markers;
pub mod store;

// Deprecated: remove after refactor.
pub mod backend;
pub mod query;
pub mod traits;
// Re-export all traits.
pub use traits::*;

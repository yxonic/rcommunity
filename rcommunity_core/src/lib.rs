//! This crate contains core facilities for [rcommunity].
//!
//! [rcommunity]: ../rcommunity/index.html

// Global clippy settings.
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// Use the unstable specialization feature for marker dependent add/remove logic of reations.
#![allow(incomplete_features)]
#![feature(specialization)]

pub mod error;
pub mod store;
pub mod markers;
pub mod ops;
pub mod client;

// Re-export all traits.
pub use markers::*;

//! Defines error and result types used by this crate.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not yet implemented.")]
    NotImplemented,
    #[error("Unknown error: {0}.")]
    UnknownError(String),
}

pub type Result<T> = core::result::Result<T, Error>;

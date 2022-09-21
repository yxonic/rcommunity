//! Defines error and result types used by this crate.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not yet implemented.")]
    NotImplemented,
    #[error("Unknown error: {0}.")]
    UnknownError(String),
    #[error("Serialization error: {0}.")]
    SerializationError(crate::store::format::error::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<crate::store::format::error::Error> for Error {
    fn from(e: crate::store::format::error::Error) -> Self {
        Error::SerializationError(e)
    }
}

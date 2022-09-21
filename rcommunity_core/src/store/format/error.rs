//! Se/deserialization-related errors.

use serde::{de, ser};
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Conversion error: {0}.")]
    ConversionError(std::num::TryFromIntError),
    #[error("Unknown error: {0}.")]
    UnknownError(String),
    #[error("Encountered end-of-string unexpectedly.")]
    UnexpectedEnd,
    #[error("Serialization not supported for type.")]
    NotSupported,
    #[error("serde_json error: {0}.")]
    JsonError(serde_json::Error),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::UnknownError(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::UnknownError(msg.to_string())
    }
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<std::num::TryFromIntError> for Error {
    fn from(e: std::num::TryFromIntError) -> Self {
        Error::ConversionError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e)
    }
}

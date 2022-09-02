//! Store format types to help enforce consistent data se/desrialization.

/// Se/deserialization-related errors.
pub mod error;

mod ser;

#[cfg(test)]
mod tests;

use std::marker::PhantomData;

use serde::Serialize;

use crate::utils::typename;
use error::Result;

/// Serialize object to store key.
///
/// # Errors
/// Will return `Err` if value is not serializable as store key.
pub fn to_key<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    let mut serializer = ser::Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

#[derive(Debug)]
pub struct Placeholder<T: ?Sized> {
    phantom: PhantomData<T>,
}

impl<T: ?Sized> Placeholder<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Default for Placeholder<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized> Serialize for Placeholder<T> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> {
        serializer.serialize_str(&(typename::<T>().to_owned() + ":"))
    }
}

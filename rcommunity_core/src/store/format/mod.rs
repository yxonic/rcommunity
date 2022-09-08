//! Store format types to help enforce consistent data se/desrialization.

/// Se/deserialization-related errors.
pub mod error;

mod de;
mod ser;

#[cfg(test)]
mod tests;

use std::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Serialize};

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

/// Deserialize key structure from bytes.
///
/// # Errors
/// Will return `Err` if value cannot be deserialized properly.
pub fn from_key<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = de::Deserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

#[derive(Debug, Eq, PartialEq)]
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
        serializer.serialize_newtype_struct(typename::<T>(), &())
    }
}

impl<'de, T: ?Sized> Deserialize<'de> for Placeholder<T> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_newtype_struct(
            typename::<T>(),
            PlaceholderVisitor {
                phantom: PhantomData,
            },
        )
    }
}

struct PlaceholderVisitor<T: ?Sized> {
    phantom: PhantomData<T>,
}

impl<'de, T: ?Sized> Visitor<'de> for PlaceholderVisitor<T> {
    type Value = Placeholder<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("placeholder")
    }

    fn visit_newtype_struct<D>(self, _deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Placeholder {
            phantom: PhantomData,
        })
    }
}

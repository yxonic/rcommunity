//! Store format types to help enforce consistent data se/desrialization.

pub mod error;
mod ser;
mod de;

#[cfg(test)]
mod tests;

use std::{any::type_name, marker::PhantomData};

use serde::{de::Visitor, Deserialize, Serialize};

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

/// Serialize object to store value.
///
/// # Errors
/// Will return `Err` if value is not serialized properly.
pub fn to_value<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(value)?)
}

/// Deserialize value from bytes.
///
/// # Errors
/// Will return `Err` if value cannot be deserialized properly.
pub fn from_value<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    Ok(serde_json::from_slice(s)?)
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

#[derive(Debug, Eq, PartialEq)]
pub struct TypeName<T: ?Sized> {
    phantom: PhantomData<T>,
}

impl<T: ?Sized> TypeName<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T: ?Sized> Default for TypeName<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized> Serialize for TypeName<T> {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> {
        serializer.serialize_str(typename::<T>())
    }
}

impl<'de, T: ?Sized> Deserialize<'de> for TypeName<T> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TypeNameVisitor {
            phantom: PhantomData,
        })
    }
}

struct TypeNameVisitor<T: ?Sized> {
    phantom: PhantomData<T>,
}

impl<'de, T: ?Sized> Visitor<'de> for TypeNameVisitor<T> {
    type Value = TypeName<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("typename")
    }

    fn visit_borrowed_str<E>(self, _v: &'de str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TypeName {
            phantom: PhantomData,
        })
    }
}

fn typename<T: ?Sized>() -> &'static str {
    let full_type_name = type_name::<T>();
    full_type_name.split("::").last().unwrap_or(full_type_name)
}

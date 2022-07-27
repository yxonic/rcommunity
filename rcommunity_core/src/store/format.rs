//! Store format types to help enforce consistent data se/desrialization.

use serde::{ser, Serialize};

/// Se/deserialization-related errors.
pub mod error {
    use serde::{de, ser};
    use std::fmt::Display;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("Unknown error: {0}.")]
        UnknownError(String),
        #[error("Serializing {0} is not supported.")]
        NotSupported(String),
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
}

use error::{Error, Result};

/// Key data format that preserves the original order.
#[derive(Debug, Eq, PartialEq)]
pub struct Key(String);

impl Key {
    pub fn raw(k: impl Into<String>) -> Self {
        Key(k.into())
    }
}

impl From<Key> for String {
    fn from(k: Key) -> Self {
        k.0
    }
}

/// Custom serializer that encode object as store key.
pub struct Serializer {
    output: String,
}

/// Serialize object to store key.
///
/// # Errors
/// Will return `Err` if value is not serializable as store key.
pub fn to_key<T: Serialize>(value: &T) -> Result<Key> {
    let mut serializer = Serializer {
        output: String::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(Key(serializer.output))
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        Err(Error::NotSupported("bool".to_string()))
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        Err(Error::NotSupported("i8".to_string()))
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        Err(Error::NotSupported("i16".to_string()))
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        Err(Error::NotSupported("i32".to_string()))
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        Err(Error::NotSupported("i64".to_string()))
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }
    fn serialize_u64(self, v: u64) -> Result<()> {
        todo!("serialize u64 and preserve order");
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        Err(Error::NotSupported("f32".to_string()))
    }
    fn serialize_f64(self, v: f64) -> Result<()> {
        Err(Error::NotSupported("f64".to_string()))
    }

    fn serialize_char(self, v: char) -> Result<()> {
        Err(Error::NotSupported("char".to_string()))
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        todo!("serialize str and preserve order");
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        todo!("serialize bytes and preserve order");
    }

    fn serialize_none(self) -> Result<()> {
        todo!("serialize none");
    }
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<()> {
        self.serialize_none()
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::NotSupported("unit".to_string()))
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::NotSupported("unit".to_string()))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<()> {
        self.output += name;
        self.output += ":";
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
        todo!()
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

/// Value data format. Currently the same as JSON.
#[derive(Debug, Eq, PartialEq)]
pub struct Value(String);

impl Value {
    pub fn raw(v: impl Into<String>) -> Self {
        Value(v.into())
    }
}

impl From<Value> for String {
    fn from(k: Value) -> Self {
        k.0
    }
}

#[cfg(test)]
mod test {
    use super::to_key;

    #[test]
    fn test_key_serialization() {
        let s = to_key(&3u32).unwrap();
        assert_eq!(s.0, "test");
    }
}
//! Store format types to help enforce consistent data se/desrialization.

/// Se/deserialization-related errors.
pub mod error;

#[cfg(test)]
mod tests;

use std::ops::Deref;

use byteorder::{BigEndian, WriteBytesExt};
use serde::{ser, Serialize};

use error::{Error, Result};

/// Key data format that preserves the original order.
#[derive(Debug, Eq, PartialEq)]
pub struct Key(pub(crate) Vec<u8>);

impl Key {
    #[must_use]
    pub fn raw(k: Vec<u8>) -> Self {
        Key(k)
    }
}

impl Deref for Key {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.0
    }
}

/// Custom serializer that encode object as store key.
pub struct Serializer {
    output: Vec<u8>,
}

/// Serialize object to store key.
///
/// # Errors
/// Will return `Err` if value is not serializable as store key.
pub fn to_key<T: Serialize>(value: &T) -> Result<Key> {
    let mut serializer = Serializer { output: Vec::new() };
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

    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(Error::NotSupported("bool".to_string()))
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        // equivalent as v + 2^63
        #[allow(clippy::cast_sign_loss)]
        let v = (v as u64) ^ (1 << 63);
        self.output.write_u64::<BigEndian>(v).unwrap(); // write to Vec will never fail
        Ok(())
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
        self.output.write_u64::<BigEndian>(v).unwrap(); // write to Vec will never fail
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }
    fn serialize_f64(self, v: f64) -> Result<()> {
        let v = v.to_bits();
        #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
        let mask = (-((v >> 63) as i64) as u64) | (1 << 63);
        self.output.write_u64::<BigEndian>(v ^ mask).unwrap(); // write to Vec will never fail
        Ok(())
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        Err(Error::NotSupported("char".to_string()))
    }
    fn serialize_str(self, _v: &str) -> Result<()> {
        todo!("serialize str and preserve order");
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
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
        self.output.extend(name.as_bytes());
        self.output.push(b':');
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
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

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
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

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _key: &T) -> Result<()> {
        todo!()
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
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
        _key: &'static str,
        _value: &T,
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
        _key: &'static str,
        _value: &T,
    ) -> Result<()> {
        todo!()
    }

    fn end(self) -> Result<()> {
        todo!()
    }
}

/// Value data format. Currently the same as JSON.
#[derive(Debug, Eq, PartialEq)]
pub struct Value(pub(crate) Vec<u8>);

impl Value {
    #[must_use]
    pub fn raw(v: Vec<u8>) -> Self {
        Value(v)
    }
}

impl Deref for Value {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

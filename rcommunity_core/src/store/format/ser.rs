use byteorder::{BigEndian, WriteBytesExt};
use serde::{ser, Serialize};

use super::error::{Error, Result};

/// Custom serializer that encode object as store key.
pub struct Serializer {
    pub(crate) output: Vec<u8>,
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
        if v {
            self.serialize_str("true")
        } else {
            self.serialize_str("false")
        }
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
        self.serialize_i64(v.into())
    }
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_i64(v.into())
    }
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_i64(v.into())
    }
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_i64(v.try_into().map_err(Error::ConversionError)?)
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
        Err(Error::NotSupported)
    }
    fn serialize_str(self, v: &str) -> Result<()> {
        self.output.extend(v.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output.extend(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::NotSupported)
    }
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::NotSupported)
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

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_str(name)?;
        Ok(self)
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
        value: &T,
    ) -> Result<()> {
        self.output.push(b'_');
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
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

//! Serializer for [`FluentValue`].

use std::borrow::Cow;

use fluent::types::{FluentNumber, FluentNumberOptions};
use fluent::FluentValue;
use serde::Serializer;

use super::unsupported::Unsupported;
use super::Error;

/// Serialize into a [`FluentValue`]. The result is returned as [`Serializer::Ok`].
///
/// The supported types are:
///
/// - Strings.
/// - Booleans, with `1.0` for `true` and `0.0` for `false`.
/// - Byte slices that can be decoded as valid UTF-8 strings.
/// - Numbers, with potentially lossy conversion to [`f64`].
/// - Unit structs and variants, encoded as strings.
/// - [`Option`]s and newtypes of other supported types.
///
/// See also [`ArgsSerializer`](crate::ser::ArgsSerializer).
///
/// # Example
///
/// ```rust
/// use std::borrow::Cow;
///
/// use fluent::FluentValue;
/// use fluent_serde::ser::ValueSerializer;
/// use serde::Serialize;
///
/// let ser = ValueSerializer::new();
/// let value = "foo".serialize(ser).unwrap();
/// assert_eq!(FluentValue::String(Cow::Owned("foo".into())), value);
/// ```
#[derive(Default)]
pub struct ValueSerializer {
    _private: (),
}

impl ValueSerializer {
    /// Creates a new [`ValueSerializer`].
    pub fn new() -> Self {
        Self::default()
    }
}

macro_rules! impl_cast_num {
    (
        $(
            $f:ident ( $t:ident )
        ),*
        $(,)?
    ) => {
        $(
            fn $f (self, v: $t) -> Result<Self::Ok, Self::Error> {
                Ok(FluentValue::Number(FluentNumber::new(v as f64, FluentNumberOptions::default())))
            }
        )*
    };
}

impl Serializer for ValueSerializer {
    type Ok = FluentValue<'static>;
    type Error = Error;

    type SerializeMap = Unsupported<Self::Ok>;
    type SerializeSeq = Unsupported<Self::Ok>;
    type SerializeTuple = Unsupported<Self::Ok>;
    type SerializeTupleStruct = Unsupported<Self::Ok>;
    type SerializeTupleVariant = Unsupported<Self::Ok>;
    type SerializeStruct = Unsupported<Self::Ok>;
    type SerializeStructVariant = Unsupported<Self::Ok>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        let num = if v { 1.0 } else { 0.0 };
        Ok(FluentValue::Number(FluentNumber::new(
            num,
            FluentNumberOptions::default(),
        )))
    }

    impl_cast_num! {
        serialize_i8(i8),
        serialize_i16(i16),
        serialize_i32(i32),
        serialize_i64(i64),
        serialize_i128(i128),
        serialize_u8(u8),
        serialize_u16(u16),
        serialize_u32(u32),
        serialize_u64(u64),
        serialize_u128(u128),
        serialize_f32(f32),
        serialize_f64(f64),
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(FluentValue::String(Cow::Owned(v.to_string())))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let s = std::str::from_utf8(v).map_err(|_| Error::NonUtf8Bytes)?;
        self.serialize_str(s)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(FluentValue::None)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(FluentValue::None)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(FluentValue::String(Cow::Borrowed(name)))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(FluentValue::String(Cow::Borrowed(variant)))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnsupportedType)
    }
}

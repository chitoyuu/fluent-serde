//! Serializer for [`FluentArgs`].

use std::borrow::Cow;

use fluent::{FluentArgs, FluentValue};
use serde::ser::{SerializeMap, SerializeStruct, SerializeStructVariant};
use serde::Serializer;

use super::unsupported::Unsupported;
use super::{Error, ValueSerializer};

/// Serialize into a [`FluentArgs`]. Can be used multiple times to merge structures.
///
/// The supported types are:
///
/// - Maps from strings to [`ValueSerializer`] types.
/// - Structures of [`ValueSerializer`] types.
/// - [`Option`]s and newtypes of supported types.
///
/// See also [`ValueSerializer`](crate::ser::ValueSerializer).
///
/// # Example
///
/// ```rust
/// use std::borrow::Cow;
///
/// use fluent::FluentValue;
/// use fluent::types::{FluentNumber, FluentNumberOptions};
/// use fluent_serde::ser::ArgsSerializer;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Foo {
///     foo: i32,
/// }
///
/// #[derive(Serialize)]
/// struct Bar {
///     bar: String,
/// }
///
/// let mut ser = ArgsSerializer::new();
/// Foo { foo: 42 }.serialize(&mut ser);
/// Bar { bar: "bar".into() }.serialize(&mut ser);
/// let args = ser.done();
///
/// assert_eq!(
///     &FluentValue::Number(FluentNumber::new(42.0, FluentNumberOptions::default())),
///     args.get("foo").unwrap(),
/// );
///
/// assert_eq!(
///     &FluentValue::String(Cow::Owned("bar".into())),
///     args.get("bar").unwrap(),
/// );
/// ```
#[derive(Default)]
pub struct ArgsSerializer {
    args: FluentArgs<'static>,
}

impl ArgsSerializer {
    /// Creates a new [`ArgsSerializer`].
    pub fn new() -> Self {
        ArgsSerializer::default()
    }

    /// Creates an [`ArgsSerializer`] based on an existing argument map.
    pub fn from_existing(args: FluentArgs<'static>) -> Self {
        ArgsSerializer { args }
    }

    /// Returns the built [`FluentArgs`] value.
    pub fn done(self) -> FluentArgs<'static> {
        self.args
    }
}

impl From<FluentArgs<'static>> for ArgsSerializer {
    fn from(args: FluentArgs<'static>) -> Self {
        Self::from_existing(args)
    }
}

impl<'a> Serializer for &'a mut ArgsSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Unsupported<()>;
    type SerializeTuple = Unsupported<()>;
    type SerializeTupleStruct = Unsupported<()>;
    type SerializeTupleVariant = Unsupported<()>;
    type SerializeMap = SerMap<'a>;
    type SerializeStruct = SerStruct<'a>;
    type SerializeStructVariant = SerStructVariant<'a>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(())
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
        Ok(SerMap {
            args: &mut self.args,
            current_key: None,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(SerStruct {
            args: &mut self.args,
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(SerStructVariant {
            args: &mut self.args,
        })
    }
}

/// Map serialization interface.
pub struct SerMap<'a> {
    args: &'a mut FluentArgs<'static>,
    current_key: Option<Cow<'static, str>>,
}

impl<'a> SerializeMap for SerMap<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let value = key.serialize(ValueSerializer::new())?;

        if let FluentValue::String(key) = value {
            if self.current_key.replace(key).is_some() {
                Err(Error::InvalidSerMap)
            } else {
                Ok(())
            }
        } else {
            Err(Error::UnsupportedType)
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        if let Some(key) = self.current_key.take() {
            let value = value.serialize(ValueSerializer::new())?;
            self.args.set(key, value);
            Ok(())
        } else {
            Err(Error::InvalidSerMap)
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.current_key.is_none() {
            Ok(())
        } else {
            Err(Error::InvalidSerMap)
        }
    }
}

/// Struct serialization interface.
pub struct SerStruct<'a> {
    args: &'a mut FluentArgs<'static>,
}

impl<'a> SerializeStruct for SerStruct<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let value = value.serialize(ValueSerializer::new())?;
        self.args.set(Cow::Borrowed(key), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Struct variant serialization interface.
pub struct SerStructVariant<'a> {
    args: &'a mut FluentArgs<'static>,
}

impl<'a> SerializeStructVariant for SerStructVariant<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        let value = value.serialize(ValueSerializer::new())?;
        self.args.set(Cow::Borrowed(key), value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

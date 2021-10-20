//! Serialization.

use thiserror::Error;

mod unsupported;

pub mod args;
pub mod value;

pub use args::ArgsSerializer;
pub use value::ValueSerializer;

/// Serialization error.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("this type is unsupported")]
    UnsupportedType,
    #[error("this serializer is already used")]
    AlreadyUsed,
    #[error("input bytes do not form a valid UTF-8 encoded string")]
    NonUtf8Bytes,
    #[error("invalid call sequence of map serialization methods")]
    InvalidSerMap,
    #[error("{0}")]
    Custom(String),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Error::Custom(msg.to_string())
    }
}

//! The `FromVeekun` trait allows for conversion from the representations in
//! the Veekun CSV files to pbirch types.

use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// Abstracts the idea of creating a new instance from a CSV field.
pub trait FromVeekunField: Sized {
    type VeekunErr;

    fn from_veekun_field(field: &str) -> Result<Self, Self::VeekunErr>;
}

pub trait FromVeekun: Sized {
    type Intermediate;

    const DEFAULT: Option<Self::Intermediate> = None;

    /// Creates a new instance from the parsed CSV field value.
    fn from_veekun(value: Self::Intermediate) -> Option<Self>;
}

/// Blanket implementation for parsing `FromStr` types directly from Veekun
/// CSV files.
impl<V> FromVeekun for V
    where V: FromStr + Debug + Copy, <V as FromStr>::Err: Debug
{
    type Intermediate = V;

    fn from_veekun(value: V) -> Option<Self> {
        Some(value)
    }
}

/// An error in the Veekun CSV representation.
#[derive(Debug)]
pub enum VeekunError<V>
    where V: FromStr + Debug, <V as FromStr>::Err: Debug
{
    /// The parsed value was not valid.
    Value(V),
    /// The CSV field could not be parsed.
    Parse(V::Err),
}

impl<V> Display for VeekunError<V>
    where V: FromStr + Debug + Display, <V as FromStr>::Err: Debug + Display
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            VeekunError::Value(v)
                => write!(f, "Invalid value: {}", v),
            VeekunError::Parse(e)
                => write!(f, "{}", e),
        }
    }
}

impl<V> StdError for VeekunError<V>
    where V: FromStr + Debug + Display,
        <V as FromStr>::Err: Debug + StdError + 'static
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            VeekunError::Value(_) => None,
            VeekunError::Parse(e) => Some(e),
        }
    }
}

/// Blanket implementation for `FromVeekun` types.
impl<T> FromVeekunField for T
    where T: FromVeekun,
          <T as FromVeekun>::Intermediate: FromStr + Debug + Copy,
          <<T as FromVeekun>::Intermediate as FromStr>::Err: Debug
{
    type VeekunErr = VeekunError<T::Intermediate>;

    /// Parses the field string and passes the value to `from_veekun`.
    fn from_veekun_field(field: &str) -> Result<Self, Self::VeekunErr> {
        let value = field.parse()
            .or_else(|e| T::DEFAULT.ok_or(VeekunError::Parse(e)))?;
        Self::from_veekun(value).ok_or(VeekunError::Value(value))
    }
}

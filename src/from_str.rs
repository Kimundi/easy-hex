use std::str::FromStr;

use crate::{decode_into, FromHexError, Hex, UpperHex};

impl<T> FromStr for Hex<T>
where
    T: for<'a> From<&'a [u8]>,
{
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        decode_into(s).map(Hex)
    }
}

impl<T> FromStr for UpperHex<T>
where
    T: for<'a> From<&'a [u8]>,
{
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        decode_into(s).map(UpperHex)
    }
}

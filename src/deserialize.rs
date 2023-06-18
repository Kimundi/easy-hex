use std::{fmt, marker::PhantomData};

use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};

use crate::{decode::fast_deserialize_into, Hex, UpperHex};

struct Vis<T>(PhantomData<T>);
impl<'a, T> Visitor<'a> for Vis<T>
where
    T: for<'b> TryFrom<&'b [u8]>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a hexadecimal string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value: T = fast_deserialize_into::<T>(v)
            .map_err(|_| Error::invalid_type(Unexpected::Str(v), &self))?;
        Ok(value)
    }
}

impl<'a, T> Deserialize<'a> for Hex<T>
where
    T: for<'b> TryFrom<&'b [u8]>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_str(Vis(PhantomData)).map(Hex)
    }
}

impl<'a, T> Deserialize<'a> for UpperHex<T>
where
    T: for<'b> TryFrom<&'b [u8]>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_str(Vis(PhantomData)).map(UpperHex)
    }
}

/// Deserialize function for a hex string. Can handle the output of
/// either `Hex` or `UpperHex`.
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: for<'a> TryFrom<&'a [u8]>,
{
    Hex::<T>::deserialize(deserializer).map(|v| v.0)
}

#[cfg(test)]
mod tests {
    use serde_derive::Deserialize;

    use crate::tests::from_json;

    use super::*;

    #[derive(Deserialize)]
    struct Test {
        _a: Hex<Vec<u8>>,
        _b: UpperHex<Vec<u8>>,
        #[serde(deserialize_with = "deserialize")]
        _c: Vec<u8>,
    }

    #[test]
    fn test_from_lower() {
        let hex: Hex<_> = from_json(r#"{"data":"0199ff"}"#);
        assert_eq!(hex, Hex([1_u8, 0x99, 0xff]));
    }

    #[test]
    fn test_from_upper() {
        let hex: Hex<_> = from_json(r#"{"data":"0199FF"}"#);
        assert_eq!(hex, Hex([1_u8, 0x99, 0xff]));
    }

    #[test]
    fn test_from_mixed() {
        let hex: Hex<_> = from_json(r#"{"data":"0199fF"}"#);
        assert_eq!(hex, Hex([1_u8, 0x99, 0xff]));
    }

    #[test]
    fn test_upper_from_lower() {
        let hex: UpperHex<_> = from_json(r#"{"data":"0199ff"}"#);
        assert_eq!(hex, UpperHex([1_u8, 0x99, 0xff]));
    }

    #[test]
    fn test_upper_from_upper() {
        let hex: UpperHex<_> = from_json(r#"{"data":"0199FF"}"#);
        assert_eq!(hex, UpperHex([1_u8, 0x99, 0xff]));
    }

    #[test]
    fn test_upper_from_mixed() {
        let hex: UpperHex<_> = from_json(r#"{"data":"0199fF"}"#);
        assert_eq!(hex, UpperHex([1_u8, 0x99, 0xff]));
    }
}

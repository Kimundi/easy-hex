use std::{fmt, marker::PhantomData};

use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};

use crate::{decode::fast_deserialize, Hex, UpperHex};

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
        let value: T =
            fast_deserialize::<T>(v).map_err(|_| Error::invalid_type(Unexpected::Str(v), &self))?;
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

#[cfg(test)]
mod tests {
    use crate::tests::from_json;

    use super::*;

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

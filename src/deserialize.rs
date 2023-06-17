use std::{fmt, marker::PhantomData};

use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer,
};

use crate::{decode::fast_deserialize, Hex};

struct Vis<T, const U: bool> {
    _marker: PhantomData<T>,
}
impl<'a, T, const U: bool> Visitor<'a> for Vis<T, U>
where
    T: for<'b> TryFrom<&'b [u8]>,
{
    type Value = Hex<T, U>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a hexadecimal string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value: T =
            fast_deserialize::<T>(v).map_err(|_| Error::invalid_type(Unexpected::Str(v), &self))?;
        Ok(Hex(value))
    }
}
impl<'a, T, const U: bool> Deserialize<'a> for Hex<T, U>
where
    T: for<'b> TryFrom<&'b [u8]>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        let visitor = Vis {
            _marker: PhantomData,
        };
        deserializer.deserialize_str(visitor)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::from_json;

    use super::*;

    #[test]
    fn test_lower() {
        // TODO: make better inference interface for this
        let hex: Hex<_> = from_json(r#"{"data":"0199ff"}"#);
        assert_eq!(hex, Hex::<_>([1_u8, 0x99, 0xff]));
    }

    #[test]
    fn test_upper() {
        // TODO: make better inference interface for this
        let hex: Hex<_> = from_json(r#"{"data":"0199FF"}"#);
        assert_eq!(hex, Hex::<_>([1_u8, 0x99, 0xff]));
    }

    #[test]
    fn test_mixed() {
        // TODO: make better inference interface for this
        let hex: Hex<_> = from_json(r#"{"data":"0199fF"}"#);
        assert_eq!(hex, Hex::<_>([1_u8, 0x99, 0xff]));
    }
}

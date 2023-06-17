use serde::{Serialize, Serializer};

use crate::{encode::fast_serialize, Hex, UpperHex, LOWER, UPPER};

impl<T> Serialize for Hex<T>
where
    T: AsRef<[u8]> + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        fast_serialize::<_, _, LOWER>(&self.0, |s| serializer.serialize_str(s))
    }
}

impl<T> Serialize for UpperHex<T>
where
    T: AsRef<[u8]> + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        fast_serialize::<_, _, UPPER>(&self.0, |s| serializer.serialize_str(s))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::to_json;

    use super::*;

    #[test]
    fn test_lower() {
        let hex = Hex([1_u8, 0x99, 0xff]);

        assert_eq!(to_json(&hex), r#"{"data":"0199ff"}"#);
    }

    #[test]
    fn test_upper() {
        let hex = UpperHex([1_u8, 0x99, 0xff]);

        assert_eq!(to_json(&hex), r#"{"data":"0199FF"}"#);
    }
}

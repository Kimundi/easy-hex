use serde::{Serialize, Serializer};

use crate::{encode::fast_serialize, Hex};

impl<T, const U: bool> Serialize for Hex<T, U>
where
    T: AsRef<[u8]>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        fast_serialize::<_, _, U>(&self.0, |s| serializer.serialize_str(s))
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::to_json;

    use super::*;

    #[test]
    fn test_lower() {
        // TODO: make better inference interface for this
        let hex = Hex::<_, false>([1_u8, 0x99, 0xff]);

        assert_eq!(to_json(hex), r#"{"data":"0199ff"}"#);
    }

    #[test]
    fn test_upper() {
        // TODO: make better inference interface for this
        let hex = Hex::<_, true>([1_u8, 0x99, 0xff]);

        assert_eq!(to_json(hex), r#"{"data":"0199FF"}"#);
    }
}

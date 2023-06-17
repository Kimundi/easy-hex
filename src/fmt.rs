use std::fmt;

use crate::{encode::fast_serialize, Hex, UpperHex, LOWER, UPPER};

macro_rules! impl_fmt {
    ($Hex:ident, $Trait:ident, $case:ident, $FmtTrait:ident) => {
        impl<T> fmt::$Trait for $Hex<T>
        where
            T: AsRef<[u8]> + ?Sized,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fast_serialize::<_, _, $case>(&self.0, |s| fmt::$FmtTrait::fmt(s, f))
            }
        }
    };
}

impl_fmt!(Hex, Display, LOWER, Display);
impl_fmt!(Hex, Debug, LOWER, Debug);
impl_fmt!(Hex, LowerHex, LOWER, Display);
impl_fmt!(Hex, UpperHex, UPPER, Display);

impl_fmt!(UpperHex, Display, UPPER, Display);
impl_fmt!(UpperHex, Debug, UPPER, Debug);
impl_fmt!(UpperHex, LowerHex, LOWER, Display);
impl_fmt!(UpperHex, UpperHex, UPPER, Display);

#[test]
fn test_lower() {
    let hex = Hex([1_u8, 0x99, 0xff]);

    assert_eq!(format!("{}", hex), "0199ff");
    assert_eq!(format!("{:?}", hex), "\"0199ff\"");
    assert_eq!(format!("{:x}", hex), "0199ff");
    assert_eq!(format!("{:X}", hex), "0199FF");
}

#[test]
fn test_upper() {
    let hex = UpperHex([1_u8, 0x99, 0xff]);

    assert_eq!(format!("{}", hex), "0199FF");
    assert_eq!(format!("{:?}", hex), "\"0199FF\"");
    assert_eq!(format!("{:x}", hex), "0199ff");
    assert_eq!(format!("{:X}", hex), "0199FF");
}

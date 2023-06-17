use std::fmt;

use crate::{encode::fast_serialize, Hex};

// --- formatting traits ----------------
impl<T, const U: bool> fmt::Display for Hex<T, U>
where
    T: ?Sized + AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fast_serialize::<_, _, U>(&self.0, |s| fmt::Display::fmt(s, f))
    }
}
impl<T, const U: bool> fmt::Debug for Hex<T, U>
where
    T: ?Sized + AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fast_serialize::<_, _, U>(&self.0, |s| fmt::Debug::fmt(s, f))
    }
}
impl<T, const U: bool> fmt::LowerHex for Hex<T, U>
where
    T: ?Sized + AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fast_serialize::<_, _, false>(&self.0, |s| fmt::Display::fmt(s, f))
    }
}
impl<T, const U: bool> fmt::UpperHex for Hex<T, U>
where
    T: ?Sized + AsRef<[u8]>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fast_serialize::<_, _, true>(&self.0, |s| fmt::Display::fmt(s, f))
    }
}

#[test]
fn test_lower() {
    // TODO: make better inference interface for this
    let hex = Hex::<_, false>([1_u8, 0x99, 0xff]);

    assert_eq!(format!("{}", hex), "0199ff");
    assert_eq!(format!("{:?}", hex), "\"0199ff\"");
    assert_eq!(format!("{:x}", hex), "0199ff");
    assert_eq!(format!("{:X}", hex), "0199FF");
}

#[test]
fn test_upper() {
    // TODO: make better inference interface for this
    let hex = Hex::<_, true>([1_u8, 0x99, 0xff]);

    assert_eq!(format!("{}", hex), "0199FF");
    assert_eq!(format!("{:?}", hex), "\"0199FF\"");
    assert_eq!(format!("{:x}", hex), "0199ff");
    assert_eq!(format!("{:X}", hex), "0199FF");
}

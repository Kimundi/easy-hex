use std::fmt;

use crate::Hex;

// --- formatting traits ----------------
impl<T, const U: bool> fmt::Display for Hex<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl<T, const U: bool> fmt::Debug for Hex<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl<T, const U: bool> fmt::LowerHex for Hex<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl<T, const U: bool> fmt::UpperHex for Hex<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

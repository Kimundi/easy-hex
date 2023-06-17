mod decode;
mod deserialize;
mod encode;
mod fmt;
mod serialize;

use std::ops::{Deref, DerefMut};

use bytemuck::{Pod, TransparentWrapper, Zeroable};

/// Simple wrapper around a sequence of bytes `T` that will be serialized,
/// deserialized and formatted as a hexadecimal string.
///
/// The default bool parameter `U` controls wether the string should output in uppercase:
/// - `U = true`: upper-case
/// - `U = false`: lower-case
///
/// The type also implements all relevant `bytemuck` traits, which allows
/// creating it from references to `T` without taking ownership.
#[derive(
    Copy, Clone, TransparentWrapper, Default, PartialOrd, Ord, Hash, Eq, PartialEq, Pod, Zeroable,
)]
#[repr(transparent)]
#[transparent(T)]
pub struct Hex<T, const U: bool = false>(pub T);

// --- methods ----------------
impl<T, const U: bool> Hex<T, U> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
    /// Create this from a reference to T.
    /// For more similar conversions use the bytemuck API.
    pub fn from_ref(v: &T) -> &Self {
        TransparentWrapper::wrap_ref(v)
    }
    /// Create this from a mutable reference to T.
    /// For more similar conversions use the bytemuck API.
    pub fn from_mut(v: &mut T) -> &mut Self {
        TransparentWrapper::wrap_mut(v)
    }
}

// --- conversion traits ----------------
impl<T, const U: bool> Deref for Hex<T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T, const U: bool> DerefMut for Hex<T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T, V, const U: bool> AsRef<V> for Hex<T, U>
where
    T: AsRef<V>,
{
    fn as_ref(&self) -> &V {
        self.0.as_ref()
    }
}
impl<T, const U: bool> From<T> for Hex<T, U> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

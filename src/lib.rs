mod decode;
mod deserialize;
mod encode;
mod fmt;
mod serialize;

#[cfg(test)]
mod tests;

use std::ops::{Deref, DerefMut};

use bytemuck::{Pod, TransparentWrapper, Zeroable};

/// Lowercase Hex of bytes `T`.
///
/// This is a simple wrapper around a sequence of bytes `T` that will be serialized,
/// deserialized and formatted as a lowercase hexadecimal string.
///
/// The type has a transparent representation, and implements the
/// relevant `bytemuck` traits, which allows using it even in situations
/// where you do not have ownership of the `T`.
#[derive(
    Copy, Clone, TransparentWrapper, Default, PartialOrd, Ord, Hash, Eq, PartialEq, Pod, Zeroable,
)]
#[repr(transparent)]
#[transparent(T)]
pub struct Hex<T: ?Sized>(pub T);

/// Uppercase Hex of bytes `T`.
///
/// This is a simple wrapper around a sequence of bytes `T` that will be serialized,
/// deserialized and formatted as a uppercase hexadecimal string.
///
/// The type has a transparent representation, and implements the
/// relevant `bytemuck` traits, which allows using it even in situations
/// where you do not have ownership of the `T`.
#[derive(
    Copy, Clone, TransparentWrapper, Default, PartialOrd, Ord, Hash, Eq, PartialEq, Pod, Zeroable,
)]
#[repr(transparent)]
#[transparent(T)]
pub struct UpperHex<T: ?Sized>(pub T);

macro_rules! impl_basic {
    ($Hex:ident) => {
        // --- methods ----------------
        impl<T> $Hex<T> {
            pub fn new(v: T) -> Self {
                Self(v)
            }

            pub fn into_inner(self) -> T {
                self.0
            }
        }

        // --- conversion traits ----------------
        //
        // deref
        impl<T> Deref for $Hex<T>
        where
            T: ?Sized,
        {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<T> DerefMut for $Hex<T>
        where
            T: ?Sized,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        // .as
        impl<T, V> AsRef<V> for $Hex<T>
        where
            T: ?Sized + AsRef<V>,
            V: ?Sized,
        {
            fn as_ref(&self) -> &V {
                self.0.as_ref()
            }
        }
        impl<T, V> AsMut<V> for $Hex<T>
        where
            T: ?Sized + AsMut<V>,
            V: ?Sized,
        {
            fn as_mut(&mut self) -> &mut V {
                self.0.as_mut()
            }
        }

        // from/into
        impl<T> From<T> for $Hex<T> {
            fn from(value: T) -> Self {
                Self(value)
            }
        }
        impl<'a, T> From<&'a T> for &'a $Hex<T>
        where
            T: ?Sized,
        {
            fn from(value: &'a T) -> Self {
                TransparentWrapper::wrap_ref(value)
            }
        }
        impl<'a, T> From<&'a mut T> for &'a mut $Hex<T>
        where
            T: ?Sized,
        {
            fn from(value: &'a mut T) -> Self {
                TransparentWrapper::wrap_mut(value)
            }
        }
    };
}

impl_basic!(Hex);
impl_basic!(UpperHex);

// Helper constants to make the usage of bools easier tor ead in thsi crate
pub(crate) const LOWER: bool = false;
pub(crate) const UPPER: bool = true;

#![doc = include_str!("../README.md")]

#[cfg(feature = "serde")]
mod decode;
#[cfg(feature = "serde")]
mod deserialize;
mod encode;
mod fmt;
#[cfg(feature = "serde")]
mod serialize;

#[cfg(test)]
mod tests;

use std::ops::{Deref, DerefMut};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, TransparentWrapper, Zeroable};

/// Lowercase Hex of bytes `T`.
///
/// This is a simple wrapper around a sequence of bytes `T` that will be serialized,
/// deserialized and formatted as a lowercase hexadecimal string.
///
/// The type has a transparent representation, and implements the
/// relevant `bytemuck` traits, which allows using it even in situations
/// where you do not have ownership of the `T`.
#[derive(Copy, Clone, Default, PartialOrd, Ord, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(TransparentWrapper, Pod, Zeroable))]
#[cfg_attr(feature = "bytemuck", transparent(T))]
#[repr(transparent)]
pub struct Hex<T: ?Sized>(pub T);

/// Uppercase Hex of bytes `T`.
///
/// This is a simple wrapper around a sequence of bytes `T` that will be serialized,
/// deserialized and formatted as a uppercase hexadecimal string.
///
/// The type has a transparent representation, and implements the
/// relevant `bytemuck` traits, which allows using it even in situations
/// where you do not have ownership of the `T`.
#[derive(Copy, Clone, Default, PartialOrd, Ord, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "bytemuck", derive(TransparentWrapper, Pod, Zeroable))]
#[cfg_attr(feature = "bytemuck", transparent(T))]
#[repr(transparent)]
pub struct UpperHex<T: ?Sized>(pub T);

macro_rules! impl_basic {
    ($Hex:ident) => {
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
        #[cfg(feature = "bytemuck")]
        impl<'a, T> From<&'a T> for &'a $Hex<T>
        where
            T: ?Sized,
        {
            fn from(value: &'a T) -> Self {
                TransparentWrapper::wrap_ref(value)
            }
        }
        #[cfg(feature = "bytemuck")]
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

/// Extension trait to make it more convenient to wrap byte
/// sequence types with `Hex` and `UpperHex`. All methods this provides
/// are also available via `From` implementations for `Hex` and `UpperHex`.
pub trait HexExt {
    fn into_hex(self) -> Hex<Self>;
    #[cfg(feature = "bytemuck")]
    fn as_hex(&self) -> &Hex<Self>;
    #[cfg(feature = "bytemuck")]
    fn as_hex_mut(&mut self) -> &mut Hex<Self>;

    fn into_upper_hex(self) -> UpperHex<Self>;
    #[cfg(feature = "bytemuck")]
    fn as_upper_hex(&self) -> &UpperHex<Self>;
    #[cfg(feature = "bytemuck")]
    fn as_upper_hex_mut(&mut self) -> &mut UpperHex<Self>;
}

impl<T> HexExt for T
where
    T: AsRef<[u8]> + for<'a> TryFrom<&'a [u8]>,
{
    fn into_hex(self) -> Hex<Self> {
        self.into()
    }

    #[cfg(feature = "bytemuck")]
    fn as_hex(&self) -> &Hex<Self> {
        self.into()
    }

    #[cfg(feature = "bytemuck")]
    fn as_hex_mut(&mut self) -> &mut Hex<Self> {
        self.into()
    }

    fn into_upper_hex(self) -> UpperHex<Self> {
        self.into()
    }

    #[cfg(feature = "bytemuck")]
    fn as_upper_hex(&self) -> &UpperHex<Self> {
        self.into()
    }

    #[cfg(feature = "bytemuck")]
    fn as_upper_hex_mut(&mut self) -> &mut UpperHex<Self> {
        self.into()
    }
}

/// Module that contains the serialization and deserialization
/// functions for `Hex`. Can be used with `#[serde(with = "...")]`.
#[cfg(feature = "serde")]
pub mod serde {
    pub use crate::deserialize::deserialize;
    pub use crate::serialize::serialize;
}

/// Module that contains the serialization and deserialization
/// functions for `UpperHex`. Can be used with `#[serde(with = "...")]`.
#[cfg(feature = "serde")]
pub mod serde_upper {
    pub use crate::deserialize::deserialize;
    pub use crate::serialize::serialize_upper as serialize;
}

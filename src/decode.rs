use hex::FromHexError;

use crate::encode::SMALL_SER_LEN;

pub(crate) const SMALL_DES_LEN: usize = SMALL_SER_LEN / 2;

fn fast_deserialize<V>(v: &str, out: impl FnOnce(&[u8]) -> V) -> Result<V, FromHexError> {
    if v.len() % 2 != 0 {
        return Err(FromHexError::OddLength);
    }

    let byte_len = v.len() / 2;
    let mut array;
    let mut vec;
    let buf;
    if byte_len <= SMALL_DES_LEN {
        array = [0; SMALL_DES_LEN];
        buf = &mut array[..byte_len];
    } else {
        vec = vec![0; byte_len];
        buf = &mut vec[..];
    }
    hex::decode_to_slice(v, buf)?;

    Ok(out(buf))
}

pub(crate) fn fast_deserialize_into<T>(v: &str) -> Result<T, FromHexError>
where
    T: for<'a> TryFrom<&'a [u8]>,
{
    fast_deserialize(v, |buf| {
        T::try_from(buf).map_err(|_| FromHexError::InvalidStringLength)
    })?
}

/// Decodes a hex string to a byte container.
///
/// This accepts both lower and upper case strings.
pub fn decode_into<T>(v: &str) -> Result<T, FromHexError>
where
    T: for<'a> TryFrom<&'a [u8]>,
{
    fast_deserialize_into(v)
}

/// Decodes a hex string to a byte slice.
///
/// This accepts both lower and upper case strings.
/// The resulting byte slice is passed to the closure.
pub fn decode<V>(v: &str, out: impl FnOnce(&[u8]) -> V) -> Result<V, FromHexError> {
    fast_deserialize(v, out)
}

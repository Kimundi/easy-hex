use hex::FromHexError;

use crate::encode::SMALL_SER_LEN;

pub(crate) const SMALL_DES_LEN: usize = SMALL_SER_LEN / 2;

/// Decodes a hex string to a sequence of bytes.
///
/// This accepts both lower and upper case strings.
pub fn decode<T>(v: &str) -> Result<T, FromHexError>
where
    T: for<'a> TryFrom<&'a [u8]>,
{
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

    T::try_from(buf).map_err(|_| FromHexError::InvalidStringLength)
}

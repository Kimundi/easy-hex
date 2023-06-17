use hex::FromHexError;

pub(crate) const SMALL_DES_LEN: usize = 64;

/// deserialize a hex string to bytes, using a stack buffer for small
/// hex strings.
pub(crate) fn fast_deserialize<T: for<'a> TryFrom<&'a [u8]>>(v: &str) -> Result<T, FromHexError> {
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

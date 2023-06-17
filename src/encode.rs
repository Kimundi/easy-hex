use hex::FromHexError;

pub(crate) const SMALL_SER_LEN: usize = 128;

/// deserialize a hex string to bytes, using a stack buffer for small
/// hex strings.
pub(crate) fn fast_serialize<T, V, const U: bool>(v: &T, out: impl FnOnce(&str) -> V) -> V
where
    T: ?Sized + AsRef<[u8]>,
{
    let v = v.as_ref();

    let str_len = v.len() * 2;
    let mut array;
    let mut vec;
    let buf;
    if str_len <= SMALL_SER_LEN {
        array = [0; SMALL_SER_LEN];
        buf = &mut array[..str_len];
    } else {
        vec = vec![0; str_len];
        buf = &mut vec[..];
    }

    let alpha = if U { HEX_CHARS_UPPER } else { HEX_CHARS_LOWER };
    // NB: This can never fail, as we ensure the buffer has the rigth size
    let _ = encode_to_slice(v, buf, alpha);

    // SAFTEY: buffer will only contain ASCII bytes
    let s: &str = unsafe { std::str::from_utf8_unchecked(buf) };
    out(s)
}

// --- code taken from hex crate ----------------

const HEX_CHARS_LOWER: &[u8; 16] = b"0123456789abcdef";
const HEX_CHARS_UPPER: &[u8; 16] = b"0123456789ABCDEF";

/// taken from hex crate
fn encode_to_slice<T: AsRef<[u8]>>(
    input: T,
    output: &mut [u8],
    alpha: &[u8; 16],
) -> Result<(), FromHexError> {
    if input.as_ref().len() * 2 != output.len() {
        return Err(FromHexError::InvalidStringLength);
    }

    for (byte, (i, j)) in input
        .as_ref()
        .iter()
        .zip(generate_iter(input.as_ref().len() * 2))
    {
        let (high, low) = byte2hex(*byte, alpha);
        output[i] = high;
        output[j] = low;
    }

    Ok(())
}

/// taken from hex crate
/// generates an iterator like this
/// (0, 1)
/// (2, 3)
/// (4, 5)
/// (6, 7)
/// ...
#[inline]
fn generate_iter(len: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..len).step_by(2).zip((0..len).skip(1).step_by(2))
}

/// taken from hex crate
/// the inverse of `val`.
#[inline]
#[must_use]
fn byte2hex(byte: u8, table: &[u8; 16]) -> (u8, u8) {
    let high = table[((byte & 0xf0) >> 4) as usize];
    let low = table[(byte & 0x0f) as usize];

    (high, low)
}

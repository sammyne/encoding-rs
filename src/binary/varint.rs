use std::io::{self, Read};

use crate::Error;

/// `MaxVarintLen16` is the maximum length of a varint-encoded 16-bit integer.
pub const MAX_VARINT_LEN16: usize = 3;
/// `MaxVarintLen32` is the maximum length of a varint-encoded 32-bit integer.
pub const MAX_VARINT_LEN32: usize = 5;
/// `MaxVarintLen64` is the maximum length of a varint-encoded 64-bit integer.
pub const MAX_VARINT_LEN64: usize = 10;

/// Encodes a uint64 into `buf` and returns the number of bytes written.
/// If the buffer is too small, `put_uvarint` will panic.
pub fn put_uvarint(buf: &mut [u8], x: u64) -> usize {
    let mut i = 0usize;

    let mut x = x;
    while x >= 0x80 {
        buf[i] = (x as u8) | 0x80;
        x >>= 7;
        i += 1;
    }
    buf[i] = x as u8;

    i + 1
}

/// Encodes an int64 into `buf` and returns the number of bytes written.
/// If the buffer is too small, `put_varint` will panic.
pub fn put_varint(buf: &mut [u8], x: i64) -> usize {
    // @see ZigZag encoding as https://developers.google.com/protocol-buffers/docs/encoding#signed-integers
    // h(x) = (x<<1) ^ (x>>31), where '>>' is arithmetic right shift preserving sign bit
    let ux = if x >= 0 {
        (x as u64) << 1
    } else {
        !((x as u64) << 1)
    };

    put_uvarint(buf, ux)
}

/// Reads an encoded unsigned integer from `r` and returns it as a uint64.
pub fn read_uvarint<R>(r: R) -> Result<u64, Error>
where
    R: Read,
{
    let mut x = 0u64;
    let mut s = 0u32;

    let mut n = 0usize; // #(bytes) read
    for v in r.bytes() {
        let b = v.map_err(|err| Error::IO(err, n))?;
        if b < 0x80 {
            if n > 9 || (n == 9 && b > 1) {
                return Err(Error::Overflow);
            }

            let (xx, _) = (b as u64).overflowing_shl(s);
            return Ok(x | xx);
        }

        let (xx, _) = ((b & 0x7f) as u64).overflowing_shl(s);
        x |= xx;
        s += 7;
        n += 1;
    }

    Err(Error::IO(io::Error::from(io::ErrorKind::UnexpectedEof), n))
}

/// Reads an encoded signed integer from `r` and returns it as an int64.
pub fn read_varint<R>(r: R) -> Result<i64, Error>
where
    R: Read,
{
    let ux = read_uvarint(r)?;
    // @see ZigZag encoding as https://developers.google.com/protocol-buffers/docs/encoding#signed-integers
    let x = if ux & 1 == 0 {
        (ux >> 1) as i64
    } else {
        !((ux >> 1) as i64)
    };

    Ok(x)
}

/// Decodes a uint64 from `buf` and returns that value and the
/// number of bytes read (> 0). If an error occurred, the value is 0
/// and the number of bytes `n` is <= 0 meaning:
///
///	n == 0: buf too small
///	n  < 0: value larger than 64 bits (overflow)
///	        and -n is the number of bytes read
pub fn uvariant(buf: &[u8]) -> (u64, isize) {
    let mut x = 0u64;
    let mut s = 0u32;

    for (i, &b) in buf.iter().enumerate() {
        if b < 0x80 {
            if i > 9 || (i == 9 && b > 1) {
                return (0, -((i + 1) as isize));
            }

            let (xx, _) = (b as u64).overflowing_shl(s);
            return (x | xx, (i + 1) as isize);
        }

        let (xx, _) = ((b & 0x7f) as u64).overflowing_shl(s);
        x |= xx;
        s += 7;
    }

    (0, 0)
}

/// Decodes an int64 from `buf` and returns that value and the
/// number of bytes read (> 0). If an error occurred, the value is 0
/// and the number of bytes `n` is <= 0 with the following meaning:
//
///	n == 0: buf too small
///	n  < 0: value larger than 64 bits (overflow)
///	        and -n is the number of bytes read
pub fn variant(buf: &[u8]) -> (i64, isize) {
    let (ux, n) = uvariant(buf);
    let x = {
        let mut v = (ux >> 1) as i64;
        if ux & 1 != 0 {
            v = !v;
        }
        v
    };

    (x, n)
}

use core::convert::TryInto;
use std::fmt::{self, Display, Formatter};

/// `BigEndian` is the big-endian implementation of [ByteOrder].
pub struct BigEndian;

/// `LittleEndian` is the little-endian implementation of [ByteOrder].
pub struct LittleEndian;

/// A ByteOrder specifies how to convert byte slices into
/// 16-, 32-, or 64-bit unsigned integers.
pub trait ByteOrder: Display {
    fn uint16(src: &[u8]) -> u16;
    fn uint32(src: &[u8]) -> u32;
    fn uint64(src: &[u8]) -> u64;

    fn put_uint16(dst: &mut [u8], v: u16);
    fn put_uint32(dst: &mut [u8], v: u32);
    fn put_uint64(dst: &mut [u8], v: u64);
}

impl ByteOrder for BigEndian {
    fn uint16(src: &[u8]) -> u16 {
        u16::from_be_bytes(src[0..2].try_into().expect("unfallible"))
    }

    fn uint32(src: &[u8]) -> u32 {
        u32::from_be_bytes(src[0..4].try_into().expect("unfallible"))
    }

    fn uint64(src: &[u8]) -> u64 {
        u64::from_be_bytes(src[0..8].try_into().expect("unfallible"))
    }

    fn put_uint16(dst: &mut [u8], v: u16) {
        let _ = dst[1];

        dst[0] = ((v >> 8) & 0xff) as u8;
        dst[1] = ((v >> 0) & 0xff) as u8;
    }

    fn put_uint32(dst: &mut [u8], v: u32) {
        let _ = dst[3];

        dst[0] = ((v >> 24) & 0xff) as u8;
        dst[1] = ((v >> 16) & 0xff) as u8;
        dst[2] = ((v >> 8) & 0xff) as u8;
        dst[3] = ((v >> 0) & 0xff) as u8;
    }

    fn put_uint64(dst: &mut [u8], v: u64) {
        let _ = dst[7];

        dst[0] = ((v >> 56) & 0xff) as u8;
        dst[1] = ((v >> 48) & 0xff) as u8;
        dst[2] = ((v >> 40) & 0xff) as u8;
        dst[3] = ((v >> 32) & 0xff) as u8;
        dst[4] = ((v >> 24) & 0xff) as u8;
        dst[5] = ((v >> 16) & 0xff) as u8;
        dst[6] = ((v >> 8) & 0xff) as u8;
        dst[7] = ((v >> 0) & 0xff) as u8;
    }
}

impl Display for BigEndian {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "BigEndian")
    }
}

impl ByteOrder for LittleEndian {
    fn uint16(src: &[u8]) -> u16 {
        u16::from_le_bytes(src[0..2].try_into().expect("unfallible"))
    }

    fn uint32(src: &[u8]) -> u32 {
        u32::from_le_bytes(src[0..4].try_into().expect("unfallible"))
    }

    fn uint64(src: &[u8]) -> u64 {
        u64::from_le_bytes(src[0..8].try_into().expect("unfallible"))
    }

    fn put_uint16(dst: &mut [u8], v: u16) {
        let _ = dst[1];

        dst[0] = ((v >> 0) & 0xff) as u8;
        dst[1] = ((v >> 8) & 0xff) as u8;
    }

    fn put_uint32(dst: &mut [u8], v: u32) {
        let _ = dst[3];

        dst[0] = ((v >> 0) & 0xff) as u8;
        dst[1] = ((v >> 8) & 0xff) as u8;
        dst[2] = ((v >> 16) & 0xff) as u8;
        dst[3] = ((v >> 24) & 0xff) as u8;
    }

    fn put_uint64(dst: &mut [u8], v: u64) {
        let _ = dst[7];

        dst[0] = ((v >> 0) & 0xff) as u8;
        dst[1] = ((v >> 8) & 0xff) as u8;
        dst[2] = ((v >> 16) & 0xff) as u8;
        dst[3] = ((v >> 24) & 0xff) as u8;
        dst[4] = ((v >> 32) & 0xff) as u8;
        dst[5] = ((v >> 40) & 0xff) as u8;
        dst[6] = ((v >> 48) & 0xff) as u8;
        dst[7] = ((v >> 56) & 0xff) as u8;
    }
}

impl Display for LittleEndian {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "LittleEndian")
    }
}

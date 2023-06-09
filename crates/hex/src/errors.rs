#[derive(Debug)]
pub enum Error {
    /// ErrLength reports an attempt to decode an odd-length input
    /// using [decode][crate::hex::decode] or [decode_string][crate::hex::decode_string].
    ErrLength(u32),
    /// InvalidByteError values describe errors resulting from an invalid byte in a hex string.
    InvalidByteError(usize, u8),
}

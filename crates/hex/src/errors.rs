/// Errors about decoding/encoding.
#[derive(thiserror::Error, Clone, Debug)]
pub enum Error {
    /// ErrLength reports an attempt to decode an odd-length input
    /// using [decode][crate::hex::decode] or [decode_string][crate::hex::decode_string].
    #[error("odd length of hex string")]
    Length,
    /// InvalidByteError values describe errors resulting from an invalid byte in a hex string.
    #[error("invalid byte: {}", *.0 as char)]
    InvalidByte(u8),
}

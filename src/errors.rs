use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("base64: illegal base64 data at input byte {0}")]
    CorruputInputError(usize),
    #[error("IO error: with {1} elements read/written")]
    IO(io::Error, usize),
    #[error("binary: varint overflows a 64-bit integer")]
    Overflow,
}

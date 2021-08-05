use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown errror")]
    Unknown,
    #[error("illegal {0} data at input byte {1}")]
    CorruputInputError(&'static str, usize),
    #[error("IO error: with {1} elements read/written")]
    IO(io::Error, usize),
    #[error("binary: varint overflows a 64-bit integer")]
    Overflow,
}

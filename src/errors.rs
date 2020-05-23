use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: with {1} elements read/written")]
    IO(io::Error, usize),
    #[error("binary: varint overflows a 64-bit integer")]
    Overflow,
}

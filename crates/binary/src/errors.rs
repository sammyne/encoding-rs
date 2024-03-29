use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown errror")]
    Unknown,
    #[error("illegal {0} data at input byte {1}")]
    CorruptInputError(&'static str, usize),
    #[error("IO error: with {1} elements read/written")]
    IO(io::Error, usize),
    #[error("binary: varint overflows a 64-bit integer")]
    Overflow,
}

impl From<Error> for io::Error {
    fn from(value: Error) -> Self {
        if let Error::IO(err, _) = value {
            return err;
        }

        io::Error::new(io::ErrorKind::Other, value)
    }
}

use std::convert::Into;
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

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        if let Error::IO(err, _) = self {
            return err;
        }

        io::Error::new(io::ErrorKind::Other, self.to_string())
    }
}

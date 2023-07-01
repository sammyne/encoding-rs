//use std::convert::Into;
//use std::io;

//#[derive(thiserror::Error, Debug)]
//pub enum Error {
//    #[error("illegal {0} data at input byte {1}")]
//    CorruptInputError(&'static str, usize),
//    //#[error("IO error: with {1} elements read/written")]
//    //IO(io::Error, usize),
//}
//
//impl Into<io::Error> for Error {
//    fn into(self) -> io::Error {
//        if let Error::IO(err, _) = self {
//            return err;
//        }
//
//        io::Error::new(io::ErrorKind::Other, self.to_string())
//    }
//}

use std::{error::Error, fmt::Display};

/// Error occurs during decoding.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CorruptInputError {
    pub c: Option<u8>,
    pub idx: usize,
    pub written: usize,
}

impl CorruptInputError {
    pub(crate) fn new(src: &[u8], idx: usize, written: usize) -> Self {
        let c = src.get(idx).map(|v| *v);
        Self { c, idx, written }
    }
}

impl Display for CorruptInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = self.c {
            write!(
                f,
                "illegal base64 data '{}' at input byte {} after writing {} bytes",
                c.escape_ascii().to_string(),
                self.idx,
                self.written
            )
        } else {
            write!(
                f,
                "illegal base64 data at input byte {} after writing {} bytes",
                self.idx, self.written
            )
        }
    }
}

impl Error for CorruptInputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

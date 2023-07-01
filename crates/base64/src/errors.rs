use std::{error::Error, fmt::Display};

/// Error occurs during decoding.
#[derive(Debug, Default, Clone, Copy)]
pub struct CorruptInputError {
    /// Corrupted character if any.
    pub c: Option<u8>,
    /// Index of the corrupted character.
    pub idx: usize,
    /// Number of bytes has been written to destination buffer.
    pub written: usize,
}

impl CorruptInputError {
    pub(crate) fn new(src: &[u8], idx: usize, written: usize) -> Self {
        let c = src.get(idx).copied();
        Self { c, idx, written }
    }
}

impl Display for CorruptInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = self.c {
            write!(
                f,
                "illegal base64 data '{}' at input byte {} after writing {} bytes",
                c.escape_ascii(),
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

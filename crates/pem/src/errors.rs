use std::io;

/// Possible errors during encoding and decoding.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// header key contains colons.
    #[error("cannot encode a header key that contains a colon")]
    BadHeaderKey,
    /// IO error reported by wrapped writer.
    #[error("io: {0}")]
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

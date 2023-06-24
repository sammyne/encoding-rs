use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("cannot encode a header key that contains a colon")]
    BadHeaderKey,
    #[error("io: {0}")]
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

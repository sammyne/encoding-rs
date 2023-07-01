//! Implements the ascii85 data encoding
//! as used in the btoa tool and Adobe's PostScript and PDF document formats.

mod decoder;
mod encoder;
mod encoding;
mod errors;

pub use decoder::*;
pub use encoder::*;
pub use encoding::*;
pub use errors::CorruptInputError;

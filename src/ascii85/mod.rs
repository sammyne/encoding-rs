//! Module ascii85 implements the ascii85 data encoding
//! as used in the btoa tool and Adobe's PostScript and PDF document formats.

mod decoder;
mod encoder;
mod encoding;

#[cfg(test)]
mod tests;

pub use self::encoding::*;
pub use decoder::*;
pub use encoder::*;

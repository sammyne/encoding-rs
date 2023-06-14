//! Module base64 implements base64 encoding as specified by [RFC 4648].
//!
//! [RFC 4648]: https://rfc-editor.org/rfc/rfc4648.html

mod constants;
mod decoder;
mod encoder;
mod encoding;
mod errors;

pub use constants::*;
pub use decoder::*;
pub use encoder::*;
pub use encoding::*;
pub use errors::*;

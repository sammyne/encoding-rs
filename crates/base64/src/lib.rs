//! Module base64 implements base64 encoding as specified by [RFC 4648].
//!
//! [RFC 4648]: https://rfc-editor.org/rfc/rfc4648.html

mod constants;
mod encoder;
mod errors;

pub use constants::*;
pub use encoder::*;
pub use errors::*;

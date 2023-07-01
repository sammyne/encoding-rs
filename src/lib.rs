//! encoding defines interfaces shared by other modules that convert data to and from byte-level and textual
//! representations.

pub(self) mod builtin;
mod errors;

pub mod ascii85;

pub use errors::*;

pub use base32;
pub use base64;
pub use binary;
pub use csv;
pub use hex;
pub use pem;

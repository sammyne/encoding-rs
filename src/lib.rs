//! encoding defines interfaces shared by other modules that convert data to and from byte-level and textual
//! representations.

pub(self) mod builtin;
mod errors;

pub mod ascii85;
pub mod base32;
pub mod base64;
pub mod binary;

pub use errors::*;

pub use csv;
pub use hex;

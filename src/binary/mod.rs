//! Module binary implements simple translation between numbers and byte
//! sequences and encoding and decoding of varints.
//!
//! Numbers are translated by reading and writing fixed-size values.
//! A fixed-size value is either a fixed-size arithmetic
//! type (bool, i8, u8, i16, f32, ...)
//! or an array or struct containing only fixed-size values.
//!
//! The varint functions encode and decode single integer values using
//! a variable-length encoding; smaller values require fewer bytes.
//! For a specification, see
//! <https://developers.google.com/protocol-buffers/docs/encoding>.
//!
//! This module favors simplicity over efficiency. Clients that require
//! high-performance serialization, especially for large data structures,
//! should look at more advanced solutions such as protocol buffers.

mod binary;
mod varint;

#[cfg(test)]
mod tests;

pub use binary::*;
pub use varint::*;

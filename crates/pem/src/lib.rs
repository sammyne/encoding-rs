//! Implements the PEM data encoding, which originated in Privacy
//! Enhanced Mail. The most common use of PEM encoding today is in TLS keys and
//! certificates. See [RFC 1421].
//!
//! [RFC 1421]: https://rfc-editor.org/rfc/rfc1421.html
//!

use std::collections::HashMap;
use std::io::{self, Write};

const COLON: &[u8] = b":";
const NL: &[u8] = b"\n";
const PEM_END: &[u8] = b"\n-----END ";
const PEM_END_OF_LINE: &[u8] = b"-----";
const PEM_START: &[u8] = b"\n-----BEGIN ";

/// A Block represents a PEM encoded structure.
///
/// The encoded form is:
/// ```pem
/// -----BEGIN Type-----
/// Headers
/// base64-encoded Bytes
/// -----END Type-----
/// ```
///
/// where Headers is a possibly empty sequence of `Key: Value` lines.
#[derive(Default, PartialEq, Debug, Eq)]
pub struct Block {
    /// the type, taken from the preamble (i.e. "RSA PRIVATE KEY").
    pub type_: String,
    /// Optional headers.
    pub headers: HashMap<String, String>,
    /// The decoded bytes of the contents. Typically a DER encoded ASN.1 structure.
    pub bytes: Vec<u8>,
}

impl Block {
    /// decode will find the next PEM formatted block (certificate, private key
    /// etc) in the input. It returns that block and the remainder of the input. If
    /// no PEM data is found, the whole of the input is returned in `Err`.
    ///
    /// # Example
    /// ```
    #[doc =include_str!("../examples/block_decode.rs")]
    /// ```
    pub fn decode(data: &[u8]) -> Result<(Self, &[u8]), &[u8]> {
        // PEM_START begins with a newline. However, at the very beginning of
        // the byte array, we'll accept the start string without it.
        let mut rest = data;

        loop {
            if let Some(v) = rest.strip_prefix(&PEM_START[1..]) {
                rest = v;
            } else if let Some((_, after)) = bytes::cut(rest, PEM_START) {
                // todo: improve efficiency of bytes::cut
                rest = after;
            } else {
                return Err(data);
            }

            let type_line = {
                let (type_line, remaining) = get_line(rest);
                rest = remaining;
                if !type_line.ends_with(PEM_END_OF_LINE) {
                    continue;
                }
                &type_line[..(type_line.len() - PEM_END_OF_LINE.len())]
            };

            let mut p = Block {
                type_: unsafe { std::str::from_utf8_unchecked(type_line) }.to_owned(),
                ..Default::default()
            };

            loop {
                // This loop terminates because getLine's second result is
                // always smaller than its argument.
                if rest.is_empty() {
                    return Err(data);
                }
                let (line, next) = get_line(rest);

                let (key, val) = match bytes::cut(line, COLON) {
                    Some(kv) => kv,
                    None => break,
                };

                // TODO: cope with values that spread across lines.
                let key = unsafe { String::from_utf8_unchecked(bytes::trim_space(key).to_vec()) };
                let val = unsafe { String::from_utf8_unchecked(bytes::trim_space(val).to_vec()) };
                p.headers.insert(key, val);

                rest = next;
            }

            // If there were no headers, the END line might occur
            // immediately, without a leading newline.
            let (end_index, end_trailer_index) =
                if p.headers.is_empty() && rest.starts_with(&PEM_END[1..]) {
                    (0, PEM_END.len() - 1)
                } else if let Some(i) = bytes::index(rest, PEM_END) {
                    (i, i + PEM_END.len())
                } else {
                    continue;
                };

            // After the "-----" of the ending line, there should be the same type
            // and then a final five dashes.
            let end_trailer = &rest[end_trailer_index..];
            let end_trailer_len = type_line.len() + PEM_END_OF_LINE.len();
            if end_trailer.len() < end_trailer_len {
                continue;
            }

            let rest_end_of_line = &end_trailer[end_trailer_len..];
            let end_trailer = &end_trailer[..end_trailer_len];
            if !end_trailer.starts_with(type_line) || !end_trailer.ends_with(PEM_END_OF_LINE) {
                continue;
            }

            // The line must end with only whitespace.
            let (s, _) = get_line(rest_end_of_line);
            if !s.is_empty() {
                continue;
            }

            let base64_data = remove_spaces_and_tabs(&rest[..end_index]);
            p.bytes = vec![0u8; base64::STD_ENCODING.decoded_len(base64_data.len())];
            match base64::STD_ENCODING.decode(&mut p.bytes, &base64_data) {
                Ok(n) => p.bytes.resize(n, 0),
                Err(_) => continue,
            };

            // the -1 is because we might have only matched PEM_END without the
            // leading newline if the PEM block was empty.
            let (_, remaining) = get_line(&rest[(end_index + PEM_END.len() - 1)..]);
            rest = remaining;
            return Ok((p, rest));
        }
    }
}

/// Alias of [Block::decode].
///
/// # Example
/// ```
#[doc = include_str!("../examples/decode.rs")]
/// ```
pub fn decode(data: &[u8]) -> Result<(Block, &[u8]), &[u8]> {
    Block::decode(data)
}

/// Writes the PEM encoding of `b` to `out`.
///
/// # Example
/// ```
#[doc = include_str!("../examples/encode.rs")]
/// ```
pub fn encode<W>(out: &mut W, b: &Block) -> Result<(), Error>
where
    W: Write,
{
    if b.headers.keys().any(|k| k.contains(':')) {
        return Err(Error::BadHeaderKey);
    }

    // All errors below are relayed from underlying io.Writer,
    // so it is now safe to write data.
    out.write_all(&PEM_START[1..])?;
    out.write_all((b.type_.clone() + "-----\n").as_bytes())?;

    if !b.headers.is_empty() {
        const PROC_TYPE: &str = "Proc-Type";

        let mut h = Vec::with_capacity(b.headers.len());
        let mut has_proc_type = false;
        for k in b.headers.keys() {
            if k == PROC_TYPE {
                has_proc_type = true;
                continue;
            }
            h.push(k);
        }

        // The Proc-Type header must be written first.
        // See RFC 1421, section 4.6.1.1
        if has_proc_type {
            write_header(out, PROC_TYPE, &b.headers[PROC_TYPE])?;
        }

        // For consistency of output, write other headers sorted by key.
        h.sort();
        for k in h {
            write_header(out, k, &b.headers[k])?;
        }
        out.write_all(NL)?;
    }

    base64::new_encoder(*base64::STD_ENCODING, line_breaker::new(out)).write_all(&b.bytes)?;

    out.write_all(&PEM_END[1..])?;
    out.write_all((b.type_.clone() + "-----\n").as_bytes())?;

    Ok(())
}

/// Returns the PEM encoding of `b`.
///
/// If `b` has invalid headers and cannot be encoded,
/// `encode_to_memory` returns `None`. If it is important to
/// report details about this error case, use [encode] instead.
pub fn encode_to_memory(b: &Block) -> Option<Vec<u8>> {
    let mut buf = vec![];
    if encode(&mut buf, b).is_err() {
        None
    } else {
        Some(buf)
    }
}

fn get_line(data: &[u8]) -> (&[u8], &[u8]) {
    let (i, j) = match data.iter().position(|&v| v == b'\n') {
        None => (data.len(), data.len()),
        Some(mut i) => {
            let j = i + 1;
            while (i > 0) && (data[i - 1] == b'\r') {
                i -= 1;
            }
            (i, j)
        }
    };

    let line = bytes::trim_right(&data[..i], " \t");
    (line, &data[j..])
}

fn remove_spaces_and_tabs(data: &[u8]) -> Vec<u8> {
    if data.iter().any(|v| b" \t".contains(v)) {
        // Fast path; most base64 data within PEM contains newlines, but
        // no spaces nor tabs.
        return data.to_vec();
    }

    let mut out = Vec::with_capacity(data.len());

    for &b in data {
        if std::matches!(b, b' ' | b'\t') {
            continue;
        }
        out.push(b);
    }

    out
}

fn write_header<W>(out: &mut W, k: &str, v: &str) -> Result<(), io::Error>
where
    W: Write,
{
    let kv = format!("{k}: {v}\n");
    out.write_all(kv.as_bytes())
}

mod bytes;
mod errors;
mod line_breaker;

pub use errors::*;

#[cfg(test)]
mod tests;

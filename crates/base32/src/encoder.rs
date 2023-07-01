use std::io::{self, Write};

use crate::Encoding;

struct Encoder<W>
where
    W: Write,
{
    enc: Encoding,
    w: W,
    err: Option<io::Error>,

    buf: [u8; 5],
    nbuf: usize,
    out: [u8; 1024],
}

impl<W> Encoder<W>
where
    W: Write,
{
    fn new(enc: Encoding, w: W) -> Self {
        Self {
            enc,
            w,
            err: None,
            buf: [0u8; 5],
            nbuf: 0,
            out: [0u8; 1024],
        }
    }

    fn error_or<T>(&self, ok: T) -> io::Result<T> {
        if let Some(err) = &self.err {
            Err(io::Error::new(err.kind(), err.to_string()))
        } else {
            Ok(ok)
        }
    }
}

impl<W> Drop for Encoder<W>
where
    W: Write,
{
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

impl<W> Write for Encoder<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.error_or(0)?;
        let mut buf = buf;
        let mut written = 0usize;

        // Leading fringe
        if self.nbuf > 0 {
            let mut i = 0usize;
            while i < buf.len() && self.nbuf < 5 {
                self.buf[self.nbuf] = buf[i];
                self.nbuf += 1;
                i += 1;
            }
            written += i;
            buf = &buf[i..];
            if self.nbuf < 5 {
                return Ok(written);
            }

            self.enc.encode(&mut self.out, &self.buf);
            if let Err(err) = self.w.write(&self.out[..8]) {
                self.err = Some(err);
                self.error_or(0)?;
            }
            self.nbuf = 0;
        }

        // Large interior chunks
        while buf.len() > 5 {
            let mut nn = self.out.len() / 8 * 5; // 5 bytes will encoded into 8 base32-bytes
            if nn > buf.len() {
                nn = buf.len();
                nn -= nn % 5;
            }

            self.enc.encode(self.out.as_mut(), &buf[0..nn]);
            if let Err(err) = self.w.write(&self.out[..(nn / 5 * 8)]) {
                self.err = Some(err);
                self.error_or(0)?;
            }
            written += nn;
            buf = &buf[nn..];
        }

        // Trailing fringe
        for (i, v) in buf.iter().enumerate() {
            self.buf[i] = *v;
        }
        self.nbuf = buf.len();
        written += buf.len();

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.err.is_none() && self.nbuf > 0 {
            self.enc.encode(self.out.as_mut(), &self.buf[..self.nbuf]);
            let encoded_len = self.enc.encoded_len(self.nbuf);
            self.nbuf = 0;
            if let Err(err) = self.w.write(&self.out[..encoded_len]) {
                self.err = Some(err);
            }
        }

        self.error_or(())
    }
}

/// Returns a new base32 stream encoder. Data written to
/// the returned writer will be encoded using `enc` and then written to `w`.
/// Base32 encodings operate in 5-byte blocks.
///
/// # Example
/// ```
#[doc = include_str!("../examples/encoder.rs")]
/// ```
pub fn new_encoder<W>(enc: Encoding, w: W) -> impl Write
where
    W: Write,
{
    Encoder::new(enc, w)
}

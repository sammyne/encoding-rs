use std::{
    io::{ErrorKind, Write},
    ops::Range,
};

use crate::Encoding;

struct Encoder<W>
where
    W: Write,
{
    enc: Encoding,
    w: W,
    buf: [u8; 3],
    nbuf: usize,
    out: [u8; 1024],
    out_pending: Range<usize>,
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
    fn write(&mut self, p: &[u8]) -> std::io::Result<usize> {
        if !self.out_pending.is_empty() {
            self.out_pending.start += self.w.write(&self.out[self.out_pending.clone()])?;
            return Err(ErrorKind::Interrupted.into());
        }

        let mut p = p;

        let mut n = 0; // number of bytes written

        // leading fringe
        if self.nbuf > 0 {
            let mut i = 0usize;
            while (i < p.len()) && (self.nbuf < 3) {
                self.buf[self.nbuf] = p[i];
                self.nbuf += 1;
                i += 1;
            }
            n += i;
            p = &p[i..];

            if self.nbuf < 3 {
                return Ok(n);
            }

            self.enc.encode(&mut self.out, &self.buf);
            let written = self.w.write(&self.out[..4])?;

            self.out_pending.start = written;
            self.out_pending.end = 4;
            self.nbuf = 0;

            if written < 4 {
                return Ok(n);
            }
        }

        // Large interior chunks.
        while p.len() >= 3 {
            let nn = {
                let v = (self.out.len() / 4 * 3).min(p.len());
                v - v % 3
            };

            self.enc.encode(&mut self.out, &p[..nn]);
            p = &p[nn..];
            n += nn;

            let ell = nn / 3 * 4;
            match self.w.write(&self.out[..ell]) {
                Ok(v) if v == ell => {}
                Ok(v) => {
                    self.out_pending.start = v;
                    self.out_pending.end = ell;
                    return Ok(n);
                }
                Err(err) => return if n > 0 { Ok(n) } else { Err(err) },
            }
        }

        // Trailing fringe
        builtin::copy(&mut self.buf, p);
        self.nbuf = p.len();
        n += p.len();

        Ok(n)
    }

    // Flushes any pending output from the encoder.
    // It is an error to call [Self::write] after calling flush.
    fn flush(&mut self) -> std::io::Result<()> {
        if !self.out_pending.is_empty() {
            self.out_pending.start += self.w.write(&self.out[self.out_pending.clone()])?;
        }

        if self.nbuf == 0 {
            return Ok(());
        }

        self.enc.encode(&mut self.out, &self.buf[..self.nbuf]);

        let n = self.enc.encoded_len(self.nbuf);
        self.w.write_all(&self.out[..n])?;
        self.nbuf = 0;

        Ok(())
    }
}

///
/// # Example
/// ```
#[doc = include_str!("../examples/encoder.rs")]
/// ```
pub fn new_encoder<W>(enc: Encoding, w: W) -> impl Write
where
    W: Write,
{
    Encoder {
        enc,
        w,
        buf: [0u8; 3],
        nbuf: 0,
        out: [0u8; 1024],
        out_pending: Range::default(),
    }
}

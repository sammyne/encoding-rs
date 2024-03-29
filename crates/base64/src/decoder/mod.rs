use std::io::{self, ErrorKind, Read};
use std::ops::Range;

use crate::{CorruptInputError, Encoding};

struct Decoder<R>
where
    R: Read,
{
    enc: Encoding,
    r: R,
    buf: [u8; 1024],
    nbuf: usize,
    outbuf: [u8; 1024 / 4 * 3],
    outbuf_pending: Range<usize>,
    corrupted_err: Option<CorruptInputError>,
}

struct NewlineFilteringReader<R>
where
    R: Read,
{
    wrapped: R,
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, p: &mut [u8]) -> std::io::Result<usize> {
        if !self.outbuf_pending.is_empty() {
            // Use leftover decoded output from last read.
            let n = builtin::copy(p, &self.outbuf[self.outbuf_pending.clone()]);
            self.outbuf_pending.start += n;
            return Ok(n);
        }

        if let Some(err) = self.corrupted_err {
            return Err(new_other_io_err(err));
        }

        // This code assumes that d.r strips supported whitespace ('\r' and '\n').

        // Refill buffer.
        let mut eof = false;
        while (self.nbuf < 4) && !eof {
            let nn = (p.len() / 3 * 4).max(4).min(self.buf.len());
            let nn = self.r.read(&mut self.buf[self.nbuf..nn])?;
            self.nbuf += nn;
            eof = nn == 0;
        }

        if self.nbuf < 4 {
            if self.enc.pad_char.is_none() && (self.nbuf > 0) {
                // Decode final fragment, without padding.
                let nw = match self.enc.decode(&mut self.outbuf, &self.buf[..self.nbuf]) {
                    Ok(nw) => nw,
                    Err(err) => {
                        self.corrupted_err = Some(err);
                        if err.written == 0 {
                            return Err(new_other_io_err(err));
                        }
                        err.written
                    }
                };
                self.nbuf = 0;
                let n = builtin::copy(p, &self.outbuf[..nw]);
                self.outbuf_pending.start = n;
                self.outbuf_pending.end = nw;
                if (n > 0) || (p.is_empty() && !self.outbuf_pending.is_empty()) {
                    return Ok(n);
                }
            }

            if eof && (self.nbuf > 0) {
                return Err(ErrorKind::UnexpectedEof.into());
            }

            return Ok(0);
        }

        // Decode chunk into p, or d.out and then p if p is too small.
        let nr = self.nbuf / 4 * 4;
        let nw = self.nbuf / 4 * 3;
        let (written, nr) = if nw > p.len() {
            match self.enc.decode(&mut self.outbuf, &self.buf[..nr]) {
                Ok(v) => {
                    self.outbuf_pending.start = builtin::copy(p, &self.outbuf[..v]);
                    self.outbuf_pending.end = v;
                    (self.outbuf_pending.start, nr)
                }
                Err(err) => {
                    self.corrupted_err = Some(err);
                    (err.written, err.idx)
                }
            }
        } else {
            match self.enc.decode(p, &self.buf[..nr]) {
                Ok(v) => (v, nr),
                Err(err) => {
                    self.corrupted_err = Some(err);
                    (err.written, err.idx)
                }
            }
        };

        self.nbuf -= nr;
        self.buf.copy_within(nr.., 0);

        Ok(written)
    }
}

impl<R> Read for NewlineFilteringReader<R>
where
    R: Read,
{
    fn read(&mut self, p: &mut [u8]) -> std::io::Result<usize> {
        let mut n = self.wrapped.read(p)?;

        while n > 0 {
            let mut offset = 0usize; // 1st non-ok index
            for i in 0..n {
                if std::matches!(p[i], crate::CR | crate::LF) {
                    continue;
                }
                if i != offset {
                    p[offset] = p[i];
                }
                offset += 1;
            }
            if offset > 0 {
                return Ok(offset);
            }
            n = self.wrapped.read(p)?; // Previous buffer entirely whitespace, read again
        }

        Ok(0)
    }
}

/// Constructs a new base64 stream decoder.
pub fn new_decoder<R>(enc: Encoding, r: R) -> impl Read
where
    R: Read,
{
    Decoder {
        enc,
        r: NewlineFilteringReader { wrapped: r },
        buf: [0u8; 1024],
        nbuf: 0,
        outbuf: [0u8; 1024 / 4 * 3],
        outbuf_pending: Range::default(),
        corrupted_err: None,
    }
}

fn new_other_io_err<E>(e: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, e)
}

#[cfg(test)]
mod tests;

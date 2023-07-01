use std::io::ErrorKind;
use std::{
    io::{self, Read},
    ops::Range,
};

use crate::{CorruptInputError, Encoding};

struct Decoder<R>
where
    R: Read,
{
    enc: Encoding,
    r: filter::NewlineFilteringReader<R>,

    buf: [u8; 1024],
    nbuf: usize,
    outbuf: [u8; 1024 / 8 * 5],
    outbuf_pending: Range<usize>,
    corrupted_err: Option<CorruptInputError>,
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, p: &mut [u8]) -> io::Result<usize> {
        // Use leftover decoded output from last read.
        if !self.outbuf_pending.is_empty() {
            let n = builtin::copy(p, &self.outbuf[self.outbuf_pending.clone()]);
            self.outbuf_pending.start += n;
            return Ok(n);
        }

        self.error_or(())?;

        // Read a chunk
        let nn = (p.len() / 5 * 8).max(8).min(self.buf.len());
        let mut eof = false;
        while (self.nbuf < 8) && !eof {
            let nn = self.r.read(&mut self.buf[self.nbuf..nn])?;
            self.nbuf += nn;
            eof = nn == 0;
        }

        if self.nbuf < 8 {
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
                if (n > 0) || ((p.len() == 0) && !self.outbuf_pending.is_empty()) {
                    return Ok(n);
                }
            }

            if eof && (self.nbuf > 0) {
                return Err(ErrorKind::UnexpectedEof.into());
            }

            return Ok(0);
        }

        // Decode chunk into p, or d.out and then p if p is too small.
        let nr = self.nbuf / 8 * 8;
        let nw = self.nbuf / 8 * 5;

        let (written, nr) = if nw > p.len() {
            match self.enc.decode_(&mut self.outbuf, &self.buf[..nr]) {
                Ok((v, _)) => {
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
            match self.enc.decode_(p, &self.buf[..nr]) {
                Ok((v, _)) => (v, nr),
                Err(err) => {
                    self.corrupted_err = Some(err);
                    (err.written, err.idx)
                }
            }
        };

        if written == 0 {
            self.error_or(())?;
        }

        self.nbuf -= nr;
        self.buf.copy_within(nr.., 0);

        Ok(written)
    }
}

impl<R> Decoder<R>
where
    R: Read,
{
    /// Constructs a new base32 stream decoder.
    pub fn new(enc: Encoding, r: R) -> Self {
        Self {
            enc,
            r: filter::NewlineFilteringReader::new(r),
            buf: [0u8; 1024],
            nbuf: 0,
            outbuf: [0u8; 1024 / 8 * 5],
            outbuf_pending: Default::default(),
            corrupted_err: None,
        }
    }

    fn error_or<T>(&self, ok: T) -> io::Result<T> {
        match self.corrupted_err {
            Some(err) => Err(new_other_io_err(err)),
            None => Ok(ok),
        }
    }
}

/// Constructs a new base32 stream decoder.
pub fn new_decoder<R>(enc: Encoding, r: R) -> impl Read
where
    R: Read,
{
    Decoder::new(enc, r)
}

fn new_other_io_err<E>(e: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, e)
}

mod filter;

#[cfg(test)]
mod tests;

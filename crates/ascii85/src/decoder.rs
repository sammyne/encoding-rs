use std::{
    io::{self, ErrorKind, Read},
    ops::Range,
};

use crate::CorruptInputError;

pub struct Decoder<R>
where
    R: Read,
{
    corrupted_err: Option<CorruptInputError>,
    r: R,
    buf: [u8; 1024],
    nbuf: usize,
    outbuf: [u8; 1024],
    outbuf_pending: Range<usize>,
}

impl<R> Decoder<R>
where
    R: Read,
{
    /// Constructs a new ascii85 stream decoder.
    pub fn new(r: R) -> Self {
        Self {
            corrupted_err: None,
            r,
            buf: [0u8; 1024],
            nbuf: 0,
            outbuf: [0u8; 1024],
            outbuf_pending: Range::default(),
        }
    }

    fn error_or<T>(&self, ok: T) -> io::Result<T> {
        if let Some(err) = self.corrupted_err {
            Err(io::Error::new(ErrorKind::Other, err))
        } else {
            Ok(ok)
        }
    }
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, p: &mut [u8]) -> io::Result<usize> {
        if p.len() == 0 {
            return Ok(0);
        }
        self.error_or(())?;

        let mut eof = false;
        loop {
            // Copy leftover output from last decode.
            if !self.outbuf_pending.is_empty() {
                let n = builtin::copy(p, &self.outbuf[self.outbuf_pending.clone()]);
                self.outbuf_pending.start += n;
                return Ok(n);
            }

            // Decode leftover input from last read.
            if self.nbuf > 0 {
                match crate::decode(&mut self.outbuf, &self.buf[..self.nbuf], eof) {
                    Ok((0, _)) => {
                        // Special case: input buffer is mostly filled with non-data bytes.
                        // Filter out such bytes to make room for more input.
                        let mut offset = 0usize;
                        for i in 0..self.nbuf {
                            if self.buf[i] >= b'!' {
                                self.buf[offset] = self.buf[i];
                                offset += 1;
                            }
                        }
                        self.nbuf = offset;
                    }
                    Ok((written, nsrc)) => {
                        self.outbuf_pending.start = 0;
                        self.outbuf_pending.end = written;

                        self.buf.copy_within(nsrc..self.nbuf, 0);
                        self.nbuf = self.nbuf - nsrc;

                        continue;
                    }
                    Err(err) => self.corrupted_err = Some(err),
                }
            }

            // Out of input, out of decoded output. Check errors.
            self.error_or(())?;

            // Read more data.
            let n = self.r.read(&mut self.buf[self.nbuf..])?;
            self.nbuf += n;
            eof = n == 0;

            if eof && (self.nbuf == 0) && self.outbuf_pending.is_empty() {
                return Ok(0);
            }
        }
    }
}

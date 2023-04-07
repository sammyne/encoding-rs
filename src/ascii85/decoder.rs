use core::slice;
use std::io::{self, Read};

use crate::builtin;

pub struct Decoder<R>
where
    R: Read,
{
    err: Option<io::Error>,
    read_err: Option<io::Error>,
    r: R,
    buf: [u8; 1024],
    nbuf: usize,
    outbuf: [u8; 1024],
    outbuf_start: usize,
    outbuf_end: usize,
}

impl<R> Decoder<R>
where
    R: Read,
{
    /// Constructs a new ascii85 stream decoder.
    pub fn new(r: R) -> Self {
        Self {
            err: None,
            read_err: None,
            r,
            buf: [0u8; 1024],
            nbuf: 0,
            outbuf: [0u8; 1024],
            outbuf_start: 0,
            outbuf_end: 0,
        }
    }

    fn error_or<T>(&self, ok: T) -> io::Result<T> {
        if let Some(err) = &self.err {
            Err(io::Error::new(err.kind(), err.to_string()))
        } else {
            Ok(ok)
        }
    }

    fn outbuf_ref_mut(&mut self) -> &mut [u8] {
        &mut self.outbuf[self.outbuf_start..self.outbuf_end]
    }
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }
        self.error_or(0)?;

        let mut eof = false;
        loop {
            // Copy leftover output from last decode.
            let outbuf = self.outbuf_ref_mut();
            if outbuf.len() > 0 {
                let n = builtin::copy(buf, &outbuf);
                self.outbuf_start += n;
                return Ok(n);
            }

            // Decode leftover input from last read.
            if self.nbuf > 0 {
                let mut ndst = 0usize;

                match super::decode(&mut self.outbuf, &self.buf[..self.nbuf], eof) {
                    Ok((a, nsrc)) => {
                        ndst = a;
                        if ndst > 0 {
                            self.outbuf_start = 0;
                            self.outbuf_end = a;
                            let right = unsafe {
                                let b = &self.buf[nsrc..self.nbuf];
                                slice::from_raw_parts(b.as_ptr(), b.len())
                            };
                            self.nbuf = builtin::copy(&mut self.buf, right);
                            continue;
                        }
                    }
                    Err(err) => {
                        self.err = Some(err.into());
                    }
                }
                if ndst != 0 || self.err.is_some() {
                    continue;
                }

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

            // Out of input, out of decoded output. Check errors.
            self.error_or(0)?;
            if self.read_err.is_some() {
                self.err = self.read_err.take();
                self.error_or(0)?;
            } else if eof {
                return Ok(0);
            }

            // Read more data.
            match self.r.read(&mut self.buf[self.nbuf..]) {
                Ok(n) => {
                    self.nbuf += n;
                    eof = n == 0;
                }
                Err(err) => self.read_err = Some(err),
            }
        }
    }
}

use std::io::{self, Read};

use crate::base32::Encoding;
use crate::Error;

pub struct Decoder {
    enc: Encoding,
    r: Box<dyn Read>,

    buf: [u8; 1024],
    nbuf: usize,
    outbuf: [u8; 1024 / 8 * 5],
    outstart: usize,
    outend: usize,
    err: Option<io::Error>,
    end: bool,
}

impl Read for Decoder {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.outbuf_len() > 0 {
            let n = copy(buf, self.outbuf_ref_mut());
            self.outstart += n;
            if self.outbuf_len() == 0 {
                return self.error_or(n);
            }
            return Ok(n);
        }

        if self.err.is_some() {
            return self.error_or(0);
        }

        // Read a chunk
        let mut nn = buf.len() / 5 * 8;
        if nn < 8 {
            nn = 8
        }
        if nn > self.buf.len() {
            nn = self.buf.len()
        }

        // Minimum amount of bytes that needs to be read each cycle
        let expect_padding = self.enc.is_with_padding();
        let min = if expect_padding { 1 } else { 8 - self.nbuf };

        self.nbuf += match read_encoded_data(
            self.r.as_mut(),
            self.buf[self.nbuf..nn].as_mut(),
            min,
            expect_padding,
        ) {
            Ok(n) => n,
            Err((err, n)) => {
                self.err = Some(err);
                n
            }
        };
        if self.nbuf < min {
            return self.error_or(0);
        }

        // Decode chunk into p, or d.out and then p if p is too small.
        let nr = if expect_padding {
            self.nbuf
        } else {
            self.nbuf / 8 * 8
        };
        let nw = self.enc.encoded_len(self.nbuf);

        let (n, err) = if nw > buf.len() {
            let r = self.enc.decode(self.outbuf.as_mut(), &self.buf[..nr]);
        } else {
        };

        todo!()
    }
}

impl Decoder {
    pub fn new(enc: Encoding, r: Box<dyn Read>) -> Box<dyn Read> {
        let out = Self {
            enc: enc,
            r: r,
            buf: [0u8; 1024],
            nbuf: 0,
            outbuf: [0u8; 1024 / 8 * 5],
            outstart: 0usize,
            outend: 0usize,
            err: None,
            end: false,
        };

        Box::new(out)
    }

    fn outbuf_ref_mut(&mut self) -> &mut [u8] {
        self.outbuf[self.outstart..self.outend].as_mut()
    }

    fn outbuf_len(&self) -> usize {
        self.outend - self.outstart
    }

    fn error_or<T>(&self, ok: T) -> io::Result<T> {
        if let Some(err) = &self.err {
            Err(io::Error::new(err.kind(), err.to_string()))
        } else {
            Ok(ok)
        }
    }
}

fn copy<T>(dst: &mut [T], src: &[T]) -> usize
where
    T: Copy,
{
    let ell = std::cmp::min(dst.len(), src.len());
    dst[..ell].copy_from_slice(&src[..ell]);

    ell
}

fn read_encoded_data<R>(
    r: R,
    buf: &mut [u8],
    min: usize,
    padded: bool,
) -> Result<usize, (io::Error, usize)>
where
    R: Read,
{
    todo!()
}

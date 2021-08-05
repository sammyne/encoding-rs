use std::io::{self, Write};

use crate::base32::Encoding;

pub struct Encoder {
    enc: Encoding,
    w: Box<dyn Write>,
    err: Option<io::Error>,

    buf: [u8; 5],
    nbuf: usize,
    out: [u8; 1024],
}

impl Write for Encoder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.err.is_some() {
            return self.error_or(0);
        }
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
                return self.error_or(0);
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

impl Encoder {
    pub fn new(enc: Encoding, w: Box<dyn Write>) -> Box<dyn io::Write> {
        let out = Self {
            enc: enc,
            w: w,
            err: None,
            buf: [0u8; 5],
            nbuf: 0,
            out: [0u8; 1024],
        };

        Box::new(out)
    }

    fn error_or<T>(&self, ok: T) -> io::Result<T> {
        if let Some(err) = &self.err {
            Err(io::Error::new(err.kind(), err.to_string()))
        } else {
            Ok(ok)
        }
    }
}

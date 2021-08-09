use std::io::{self, Write};

pub struct Encoder {
    err: Option<io::Error>,
    w: Box<dyn Write>,
    buf: [u8; 4],
    nbuf: usize,
    out: [u8; 1024],
}

impl Write for Encoder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.error_or(0)?;

        let mut p = buf;
        let mut written = 0usize;

        // Leading fringe.
        if self.nbuf > 0 {
            let mut i = 0usize;
            while i < p.len() && self.nbuf < 4 {
                self.buf[self.nbuf] = p[i];
                self.nbuf += 1;
                i += 1;
            }
            written += i;
            p = &p[i..];
            if self.nbuf < 4 {
                return Ok(written);
            }

            let nout = super::encode(&mut self.out, &self.buf);
            if let Err(err) = self.w.write(&self.out[..nout]) {
                self.err = Some(err);
                return Ok(written);
            }
            self.nbuf = 0;
        }

        // Large interior chunks.
        while p.len() >= 4 {
            let nn = {
                let nn = usize::min(self.out.len() / 5 * 4, p.len());
                nn - nn % 4
            };

            if nn > 0 {
                let nout = super::encode(&mut self.out, &p[..nn]);
                if let Err(err) = self.w.write(&self.out[..nout]) {
                    self.err = Some(err);
                    return Ok(written);
                }
            }

            written += nn;
            p = &p[nn..];
        }

        // Trailing fringe.
        for i in 0..p.len() {
            self.buf[i] = p[i];
        }
        self.nbuf = p.len();
        written += p.len();

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.err.is_none() && self.nbuf > 0 {
            let nout = super::encode(self.out.as_mut(), &self.buf[..self.nbuf]);
            self.nbuf = 0;
            if let Err(err) = self.w.write_all(&self.out[..nout]) {
                self.err = Some(err);
            }
        }

        self.error_or(())
    }
}

impl Encoder {
    pub fn new(w: Box<dyn Write>) -> Box<dyn Write> {
        let out = Self {
            err: None,
            w: w,
            buf: [0u8; 4],
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

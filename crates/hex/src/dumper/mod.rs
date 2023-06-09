use std::io::{Error, ErrorKind, Result, Write};

pub struct Dumper<W>
where
    W: Write,
{
    w: W,
    right_chars: [u8; 18],
    buf: [u8; 14],
    used: usize, // number of bytes in the current line
    n: usize,    // number of bytes, total
    closed: bool,
}

impl<W> Dumper<W>
where
    W: Write,
{
    pub fn new(w: W) -> Self {
        Self {
            w: w,
            right_chars: [0u8; 18],
            buf: [0u8; 14],
            used: 0,
            n: 0,
            closed: false,
        }
    }
}

impl<W> Write for Dumper<W>
where
    W: Write,
{
    fn flush(&mut self) -> Result<()> {
        if self.closed {
            return Ok(());
        }

        self.closed = true;
        if self.used == 0 {
            return Ok(());
        }

        self.buf[0] = b' ';
        self.buf[1] = b' ';
        self.buf[2] = b' ';
        self.buf[3] = b' ';
        self.buf[4] = b'|';

        let n_bytes = self.used;
        while self.used < 16 {
            let l = match self.used {
                7 => 4,
                15 => 5,
                _ => 3,
            };

            self.w.write(&self.buf[..l])?;
            self.used += 1;
        }

        self.right_chars[n_bytes] = b'|';
        self.right_chars[n_bytes + 1] = b'\n';

        self.w.write(&self.right_chars[..n_bytes + 2])?;

        Ok(())
    }

    /// @dev the claim that "If an error is returned then no bytes in the buffer were written
    /// to this writer" isn't true
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.closed {
            return Err(Error::new(
                ErrorKind::Other,
                "encoding/hex: dumper closed".to_string(),
            ));
        }

        for v in buf {
            if self.used == 0 {
                let line_idx = (self.n as u32).to_be_bytes();

                self.buf[..4].copy_from_slice(&line_idx[..]);
                super::encode(&mut self.buf[4..], &line_idx[..]);

                self.buf[12] = b' ';
                self.buf[13] = b' ';

                self.w.write(&self.buf[4..])?;
            }

            super::encode(&mut self.buf[..], &[*v]);
            self.buf[2] = b' ';

            let l = match self.used {
                7 => {
                    self.buf[3] = b' ';
                    4
                }
                15 => {
                    self.buf[3] = b' ';
                    self.buf[4] = b'|';
                    5
                }
                _ => 3,
            };

            self.w.write(&self.buf[..l])?;

            self.right_chars[self.used] = to_char(*v);
            self.used += 1;
            self.n += 1;

            if self.used == 16 {
                self.right_chars[16] = b'|';
                self.right_chars[17] = b'\n';

                self.w.write(&self.right_chars[..])?;
                self.used = 0;
            }
        }

        Ok(buf.len())
    }
}

/// Returns a string that contains a hex dump of the given data. The format
/// of the hex dump matches the output of `hexdump -C` on the command line.
pub fn dump(data: &[u8]) -> String {
    if data.len() == 0 {
        return "".to_string();
    }

    let mut buf = Vec::with_capacity((1 + (data.len() - 1) / 16) * 79);

    let mut dumper = Dumper::new(&mut buf);
    let _ = dumper.write(data);
    let _ = dumper.flush();

    String::from_utf8(buf).expect("invalid utf8 string")
}

fn to_char(b: u8) -> u8 {
    match b {
        32..=126 => b,
        _ => b'.',
    }
}

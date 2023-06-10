use std::io::Write;

use crate::BUFFER_SIZE;

struct Encoder<W>
where
    W: Write,
{
    w: W,
    out: [u8; BUFFER_SIZE],
}

impl<W> Write for Encoder<W>
where
    W: Write,
{
    fn write(&mut self, p: &[u8]) -> std::io::Result<usize> {
        let mut n = 0usize;

        let mut p = p;
        'done: while p.len() > 0 {
            let chunk_size = p.len().min(BUFFER_SIZE / 2);

            let encoded = crate::encode(&mut self.out, &p[..chunk_size]);
            let mut buf = &self.out[..encoded];

            while buf.len() > 0 {
                match self.w.write(buf) {
                    Ok(0) => break 'done,
                    Ok(written) => {
                        n += written / 2;
                        buf = &buf[written..];
                    }
                    Err(_) if n > 0 => break 'done,
                    Err(e) => return Err(e),
                }
            }

            p = &p[chunk_size..];
        }

        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// Returns an [io::Write][std::io::Write] that writes lowercase hexadecimal characters to `w`.
pub fn new_encoder<W>(w: W) -> impl Write
where
    W: Write,
{
    Encoder {
        w,
        out: [0u8; BUFFER_SIZE],
    }
}

use std::io::Read;

use crate::base32::constants;

pub struct NewlineFilteringReader<R>
where
    R: Read,
{
    wrapped: R,
}

impl<R> Read for NewlineFilteringReader<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut n = self.wrapped.read(buf)?;
        while n > 0 {
            let s = &mut buf[..n];
            let offset = strip_newlines(s);
            if offset > 0 {
                return Ok(offset);
            }
            // Previous buffer entirely whitespace, read again
            n = self.wrapped.read(buf)?;
        }

        Ok(n)
    }
}

impl<R> NewlineFilteringReader<R>
where
    R: Read,
{
    pub fn new(r: R) -> Self {
        Self { wrapped: r }
    }
}

pub fn strip_newlines(src: &mut [u8]) -> usize {
    let mut offset = 0usize;
    for i in 0..src.len() {
        let c = src[i];
        if c == constants::CR || c == constants::LF {
            continue;
        }
        src[offset] = c;
        offset += 1;
    }

    offset
}
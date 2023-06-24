use std::io::Write;

const PEM_LINE_LENGTH: usize = 64;

struct LineBreaker<'a, W>
where
    W: Write,
{
    line: [u8; PEM_LINE_LENGTH],
    used: usize,
    out: &'a mut W,
}

impl<'a, W> Write for LineBreaker<'a, W>
where
    W: Write,
{
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        if self.used + b.len() < PEM_LINE_LENGTH {
            builtin::copy(&mut self.line[self.used..], b);
            self.used += b.len();
            return Ok(b.len());
        }

        self.out.write_all(&self.line[..self.used])?;

        let excess = PEM_LINE_LENGTH - self.used;
        self.used = 0;

        let n = self.out.write(&b[..excess])?;
        if n < excess {
            return Ok(n);
        }

        self.out.write_all(crate::NL)?;

        Ok(n)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.used == 0 {
            return Ok(());
        }

        self.out.write_all(&self.line[..self.used])?;

        self.out.write_all(crate::NL)
    }
}

pub fn new<'a, W>(out: &'a mut W) -> impl Write + 'a
where
    W: Write,
{
    LineBreaker {
        line: [0u8; PEM_LINE_LENGTH],
        used: 0,
        out,
    }
}

#[cfg(test)]
mod tests;

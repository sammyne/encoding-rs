use std::io::{self, BufWriter, Write};

use crate::validator;

#[derive(thiserror::Error, Debug)]
pub enum WriteError {
    #[error("invalid field or comment delimiter")]
    InvalidDelimiter,
    #[error("io: {0}")]
    Io(#[from] io::Error),
}

pub struct Writer<W>
where
    W: Write,
{
    pub comma: char,
    pub use_crlf: bool,

    w: BufWriter<W>,
}

impl<W> Writer<W>
where
    W: Write,
{
    pub fn error(&mut self) -> Option<io::Error> {
        if let Err(err) = self.w.write(&[]) {
            return Some(err);
        }

        self.flush().err()
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }

    pub fn write<I, S, T>(&mut self, record: T) -> Result<(), WriteError>
    where
        I: Iterator<Item = S>,
        S: AsRef<str>,
        T: IntoIterator<IntoIter = I>,
    {
        if !validator::valid_delimiter(self.comma) {
            return Err(WriteError::InvalidDelimiter);
        }

        let comma = {
            let mut buf = [0u8; 4];
            self.comma.encode_utf8(&mut buf).as_bytes().to_vec()
        };

        for (n, field) in record.into_iter().enumerate() {
            let field = field.as_ref();
            if n > 0 {
                self.w.write_all(&comma).map_err(WriteError::Io)?;
            }

            if !self.field_needs_quotes(field) {
                self.w.write_all(field.as_bytes()).map_err(WriteError::Io)?;
                continue;
            }

            self.write_quoted_field(field)?;
        }

        if self.use_crlf {
            self.w.write_all(b"\r\n").map_err(WriteError::Io)
        } else {
            self.w.write_all(b"\n").map_err(WriteError::Io)
        }
    }

    pub fn write_all<T, S>(&mut self, records: &[T]) -> Result<(), WriteError>
    where
        T: AsRef<[S]>,
        S: AsRef<str>,
    {
        for v in records {
            self.write(v.as_ref())?;
        }

        self.flush().map_err(WriteError::Io)
    }

    fn field_needs_quotes(&self, field: &str) -> bool {
        match field {
            "" => return false,
            r#"\."# => return true,
            _ => {}
        }

        const QUOTING_HINT: &'static str = std::concat!('"', "\r\n");
        if field.contains(|c| (c == self.comma) || QUOTING_HINT.contains(c)) {
            return true;
        }

        field
            .chars()
            .next()
            .map(|c| c.is_whitespace())
            .unwrap_or_default()
    }

    fn write_quoted_field(&mut self, field: &str) -> Result<(), WriteError> {
        self.w.write_all(b"\"").map_err(WriteError::Io)?;

        let mut field = field.as_bytes();
        while field.len() > 0 {
            let i = field
                .iter()
                .position(|c| b"\"\r\n".contains(c))
                .unwrap_or(field.len());

            self.w.write_all(&field[..i]).map_err(WriteError::Io)?;
            field = &field[i..];

            if field.len() > 0 {
                match field[0] {
                    b'"' => self.w.write_all(br#""""#).map_err(WriteError::Io)?,
                    b'\r' => {
                        if !self.use_crlf {
                            self.w.write(&[b'\r']).map_err(WriteError::Io)?;
                        }
                    }
                    b'\n' => {
                        if self.use_crlf {
                            self.w.write_all(b"\r\n").map_err(WriteError::Io)?;
                        } else {
                            self.w.write_all(b"\n").map_err(WriteError::Io)?;
                        }
                    }
                    _ => {}
                }
                field = &field[1..];
            }
        }

        self.w.write_all(b"\"").map_err(WriteError::Io)
    }

    pub fn new(w: W) -> Self {
        Self {
            comma: ',',
            use_crlf: false,
            w: BufWriter::new(w),
        }
    }
}

pub fn new_writer<W>(w: W) -> Writer<W>
where
    W: Write,
{
    Writer::new(w)
}

#[cfg(test)]
mod tests;

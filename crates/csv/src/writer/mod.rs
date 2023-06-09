use std::io::{self, BufWriter, Write};

use crate::validator;

/// Error due to writing record.
#[derive(thiserror::Error, Debug)]
pub enum WriteError {
    /// invalid field or comment delimiter
    #[error("invalid field or comment delimiter")]
    InvalidDelimiter,
    #[error("io: {0}")]
    Io(#[from] io::Error),
}

/// A Writer writes records using CSV encoding.
//
/// As returned by [new_writer], a Writer writes records terminated by a
/// newline and uses ',' as the field delimiter. The exported fields can be
/// changed to customize the details before the first call to [write][Writer::write] or
/// [write_all][Writer::write_all].
///
/// `comma` is the field delimiter.
///
/// If `use_crlf` is true, the Writer ends each output line with `\r\n` instead of `\n`.
///
/// The writes of individual records are buffered.
/// After all data has been written, the client should call the
/// [flush][Writer::flush] method to guarantee all data has been forwarded to
/// the underlying [io::Write]. Any errors that occurred should
/// be checked by calling the [error][Writer::error] method.
///
/// # Example
/// ```
#[doc = include_str!("../../examples/writer.rs")]
/// ```
pub struct Writer<W>
where
    W: Write,
{
    /// Field delimiter (set to ',' by NewWriter)
    pub comma: char,
    /// True to use \r\n as the line terminator
    pub use_crlf: bool,

    w: BufWriter<W>,
}

impl<W> Writer<W>
where
    W: Write,
{
    /// Reports any error that has occurred during a previous [write][Self::write] or [flush][Self::flush].
    pub fn error(&mut self) -> Option<io::Error> {
        if let Err(err) = self.w.write(&[]) {
            return Some(err);
        }

        self.flush().err()
    }

    /// Writes any buffered data to the underlying [io::Write].
    /// To check if an error occurred during the [flush][Self::flush], call [error][Self::error].
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.w.flush()
    }

    /// Writes a single CSV record to w along with any necessary quoting.
    /// A record is a slice of strings with each string being one field.
    /// Writes are buffered, so [flush][Self::flush] must eventually be called to ensure
    /// that the record is written to the underlying [io::Write].
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

            // If we don't have to have a quoted field then just
            // write out the field and continue to the next field.
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

    /// Writes multiple CSV records to `self` using [write][Self::write] and then calls [flush][Self::flush],
    /// returning any error from the [flush][Self::flush].
    ///
    /// # Example
    /// ```
    #[doc = include_str!("../../examples/writer_write_all.rs")]
    /// ```
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

    /// Reports whether our field must be enclosed in quotes.
    /// Fields with a Comma, fields with a quote or newline, and
    /// fields which start with a space must be enclosed in quotes.
    /// We used to quote empty strings, but we do not anymore (as of Go 1.4).
    /// The two representations should be equivalent, but Postgres distinguishes
    /// quoted vs non-quoted empty string during database imports, and it has
    /// an option to force the quoted behavior for non-quoted CSV but it has
    /// no option to force the non-quoted behavior for quoted CSV, making
    /// CSV with quoted empty strings strictly less useful.
    /// Not quoting the empty string also makes this package match the behavior
    /// of Microsoft Excel and Google Drive.
    /// For Postgres, quote the data terminating string `\.`.
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
            // Search for special characters.
            let i = field
                .iter()
                .position(|c| b"\"\r\n".contains(c))
                .unwrap_or(field.len());

            // Copy verbatim everything before the special character.
            self.w.write_all(&field[..i]).map_err(WriteError::Io)?;
            field = &field[i..];

            // Encode the special character.
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

    /// Returns a new [Writer] that writes to w.
    pub fn new(w: W) -> Self {
        Self {
            comma: ',',
            use_crlf: false,
            w: BufWriter::new(w),
        }
    }
}

/// Returns a new [Writer] that writes to w.
///
/// [Writer::new] is preferred over this function.
pub fn new_writer<W>(w: W) -> Writer<W>
where
    W: Write,
{
    Writer::new(w)
}

#[cfg(test)]
mod tests;

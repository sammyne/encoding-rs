use std::fmt::Display;
use std::io::{self, BufRead, BufReader, Read};
use std::mem;

use crate::validator;

/// Error cause occurred during parsing records.
#[derive(thiserror::Error, Debug)]
pub enum ReadError {
    /// bare `"` in non-quoted-field
    #[error("bare \" in non-quoted-field")]
    BareQuote,
    #[error("EOF")]
    Eof,
    /// wrong number of fields
    #[error("wrong number of fields")]
    FieldCount,
    /// invalid field or comment delimiter
    #[error("invalid field or comment delimiter")]
    InvalidDelimiter,
    #[error("io: {0}")]
    Io(#[from] io::Error),
    /// extraneous or missing `"` in quoted-field
    #[error("extraneous or missing \" in quoted-field")]
    Quote,
    /// extra delimiter at end of line
    #[error("extra delimiter at end of line")]
    TrailingComma,
}

/// A ParseError is returned for parsing errors.
/// Line numbers are 1-indexed and columns are 0-indexed.
#[derive(Debug)]
pub struct ParseError {
    /// Line where the record starts
    pub start_line: usize,
    /// Line where the error occurred
    pub line: usize,
    /// Column (1-based byte index) where the error occurred
    pub column: usize,
    /// The actual error
    pub err: ReadError,
}

/// A Reader reads records from a CSV-encoded file.
///
/// As returned by [Reader::new], a Reader expects input conforming to RFC 4180.
/// The exported fields can be changed to customize the details before the
/// first call to [read][Self::read] or [read_all][Self::read_all].
///
/// The Reader converts all `\r\n` sequences in its input to plain `\n`,
/// including in multiline field values, so that the returned data does
/// not depend on which line-ending convention an input file uses.
pub struct Reader<R>
where
    R: Read,
{
    /// Comma is the field delimiter.
    /// It is set to comma (',') by [Reader::new].
    /// Comma must be a valid rune and must not be `\r`, `\n`,
    /// or the Unicode replacement character (0xFFFD).
    pub comma: char,
    // `comment`, if not `None`, is the comment character. Lines beginning with the
    // `comment` character without preceding whitespace are ignored.
    // With leading whitespace the `comment` character becomes part of the
    // field, even if `trim_leading_space` is `true`.
    // `comment` must be a valid rune and must not be `\r`, `\n`,
    // or the Unicode replacement character (0xFFFD).
    // It must also not be equal to `comma`.
    pub comment: Option<char>,
    // The number of expected fields per record.
    // If `fields_per_record` is positive, [read][Self::read] requires each record to
    // have the given number of fields. If `fields_per_record` is 0, [read][Self::read] sets it to
    // the number of fields in the first record, so that future records must
    // have the same field count. If `fields_per_record` is negative, no check is
    // made and records may have a variable number of fields.
    pub fields_per_record: Option<usize>,
    // If `lazy_quotes` is true, a quote may appear in an unquoted field and a
    // non-doubled quote may appear in a quoted field.
    pub lazy_quotes: bool,
    // If `trim_leading_space` is true, leading white space in a field is ignored.
    // This is done even if the field delimiter, `comma`, is white space.
    pub trim_leading_space: bool,

    /// An index of fields inside `record_buffer`.
    /// The i'th field ends at offset `field_indices[i]` in `record_buffer`.
    field_indices: Vec<usize>,
    /// An index of field positions for the last record returned by Read.
    field_positions: Vec<Position>,
    last_record: Vec<String>,
    /// The current line being read in the CSV file.
    num_line: usize,
    /// The input stream byte offset of the current reader position.
    offset: usize,
    r: BufReader<R>,
    /// Holds the unescaped fields, one after another.
    /// The fields can be accessed by using the indexes in `field_indexes`.
    /// E.g., For the row `a,"b","c""d",e`, recordBuffer will contain `abc"de`
    /// and fieldIndexes will contain the indexes [1, 2, 5, 6].
    record_buffer: Vec<u8>,
}

/// Holds the position of a field in the current line.
#[derive(Clone, Copy)]
struct Position {
    pub line: usize,
    pub col: usize,
}

impl ReadError {
    /// check if ReadError equals for all variants except `Io`, and check only equality of `Io`'s kind.
    pub fn equal_partially(&self, other: &Self) -> bool {
        match (self, other) {
            (ReadError::BareQuote, ReadError::BareQuote) => true,
            (ReadError::Eof, ReadError::Eof) => true,
            (ReadError::FieldCount, ReadError::FieldCount) => true,
            (ReadError::InvalidDelimiter, ReadError::InvalidDelimiter) => true,
            (ReadError::Io(x), ReadError::Io(y)) => x.kind() == y.kind(),
            (ReadError::Quote, ReadError::Quote) => true,
            (ReadError::TrailingComma, ReadError::TrailingComma) => true,
            (_, _) => false,
        }
    }
}

impl ParseError {
    /// check if all fields of ParseError equals except `err`, and `err` is check using [ReadError::equal_partially]. 
    pub fn equal_partially(&self, other: &Self) -> bool {
        (self.start_line == other.start_line)
            && (self.line == other.line)
            && (self.column == other.column)
            && self.err.equal_partially(&other.err)
    }

    fn new(start_line: usize, line: usize, column: usize, err: ReadError) -> Self {
        Self {
            start_line,
            line,
            column,
            err,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.err {
            ReadError::FieldCount => {
                return write!(f, "record on line {}: {}", self.line, self.err)
            }
            _ => {}
        }

        if self.start_line != self.line {
            write!(
                f,
                "record on line {}; parse error on line {}, column {}: {}",
                self.start_line, self.line, self.column, self.err
            )
        } else {
            write!(
                f,
                "parse error on line {}, column {}: {}",
                self.line, self.column, self.err
            )
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }

    fn description(&self) -> &str {
        "description() is deprecated; use std::fmt::Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl From<ReadError> for ParseError {
    fn from(value: ReadError) -> Self {
        Self::new(0, 0, 0, value)
    }
}

impl<R> Reader<R>
where
    R: Read,
{
    /// Returns the line and column corresponding to
    /// the start of the field with the given index in the slice most recently
    /// returned by [read][Self::read]. Numbering of lines and columns starts at 1;
    /// columns are counted in bytes, not runes.
    ///
    /// If this is called with an out-of-bounds index, it panics.
    pub fn field_pos(&self, field: usize) -> (usize, usize) {
        assert!(
            field < self.field_positions.len(),
            "out of range index passed to field_pos"
        );

        let p = &self.field_positions[field];
        (p.line, p.col)
    }

    /// Returns the input stream byte offset of the current reader
    /// position. The offset gives the location of the end of the most recently
    /// read row and the beginning of the next row.
    pub fn input_offset(&self) -> usize {
        self.offset
    }

    // Reads one record (a slice of fields) from r.
    // If the record has an unexpected number of fields,
    // `read` returns the record along with the error `ReadError::FieldCount`.
    // Except for that case, `read` always returns either a non-nil
    // record or a non-nil error, but not both.
    // If there is no data left to be read, `read` returns error `ReadError::Eof`.
    pub fn read(&mut self) -> Result<&[String], ParseError> {
        let buf = mem::take(&mut self.last_record);
        match self.read_record(Some(buf)) {
            Ok(v) => self.last_record = v,
            Err((err, v)) => {
                self.last_record = v.expect("miss buf");
                return Err(err);
            }
        };
        Ok(self.last_record.as_slice())
    }

    /// Reads all the remaining records from `r`.
    /// Each record is a slice of fields.
    /// A successful call returns no error, not err == `ReadError::Eof`. Because `read_all` is
    /// defined to read until EOF, it does not treat end of file as an error to be
    /// reported.
    pub fn read_all(&mut self) -> Result<Vec<Vec<String>>, ParseError> {
        let mut out = vec![];
        loop {
            match self.read_record(None) {
                Ok(v) => out.push(v),
                Err((err, _)) => match &err.err {
                    &ReadError::Eof => return Ok(out),
                    _ => return Err(err),
                },
            };
        }
    }

    /// Returns a new [Reader] that reads from r.
    pub fn new(r: R) -> Self {
        Self {
            comma: ',',
            comment: None,
            fields_per_record: None,
            lazy_quotes: false,
            trim_leading_space: false,
            // reuse_record: false,
            field_indices: vec![],
            field_positions: vec![],
            last_record: vec![],
            num_line: 0,
            offset: 0,
            r: BufReader::new(r),
            record_buffer: vec![],
        }
    }

    /// readLine reads the next line (with the trailing endline).
    /// If EOF is hit without a trailing endline, it will be omitted.
    /// If some bytes were read, then the error is never `ReadError::Eof`.
    /// The result is only valid until the next call to `read_line`.
    /// todo: pass line buf from outside
    fn read_line(&mut self) -> Result<String, ReadError> {
        let mut line = String::new();
        match self.r.read_line(&mut line).map_err(ReadError::Io)? {
            0 => return Err(ReadError::Eof),
            _ => {}
        }

        let read_size = line.len();

        // For backwards compatibility, drop trailing \r before EOF.
        if line.ends_with('\r') {
            line.pop();
        }

        self.num_line += 1;
        self.offset += read_size;

        // Normalize \r\n to \n on all input lines.
        let b = line.as_bytes();
        if (b.len() >= 2) && (b[b.len() - 2] == b'\r') && (b[b.len() - 1] == b'\n') {
            let _ = (line.pop(), line.pop());
            line.push('\n');
        }

        Ok(line)
    }

    /// ok((record, eof))
    fn read_record(
        &mut self,
        dst: Option<Vec<String>>,
    ) -> Result<Vec<String>, (ParseError, Option<Vec<String>>)> {
        if !validator::valid_delimiter(self.comma) {
            let err = ParseError::new(0, 0, 0, ReadError::InvalidDelimiter);
            return Err((err, dst));
        }
        match &self.comment {
            Some(v) if (self.comma == *v) || !validator::valid_delimiter(*v) => {
                let err = ParseError::new(0, 0, 0, ReadError::InvalidDelimiter);
                return Err((err, dst));
            }
            _ => {}
        }

        // Read line (automatically skipping past empty lines and any comments).
        let mut line = String::new();
        let mut err_read = None;
        while err_read.is_none() {
            match self.read_line() {
                Ok(v) => {
                    if (self.comment == v.chars().next())
                        || (v.len() == length_newline(v.as_bytes()))
                    {
                        continue; // Skip comment or empty lines
                    }
                    line = v;
                }
                Err(err) => err_read = Some(err),
            }
            break;
        }

        if let Some(ReadError::Eof) = &err_read {
            return Err((ReadError::Eof.into(), dst));
        }

        // parse each field in the record
        const QUOTE_LEN: usize = "\"".len();
        let comma_len = self.comma.len_utf8();
        let rec_line = self.num_line; // Starting line for record

        self.record_buffer.clear();
        self.field_indices.clear();
        self.field_positions.clear();

        let mut pos = Position {
            line: self.num_line,
            col: 1,
        };

        let mut line_ref = line.as_str();
        let err: Option<ParseError> = 'parse_field: loop {
            if self.trim_leading_space {
                let i = match line_ref.find(|c: char| !c.is_whitespace()) {
                    None => {
                        pos.col -= length_newline(line_ref);
                        line_ref.len()
                    }
                    Some(i) => i,
                };
                line_ref = &line_ref[i..];
                pos.col += i;
            }

            if line_ref.is_empty() || !line_ref.starts_with('"') {
                // Non-quoted string field
                let (field, i) = match line_ref.find(self.comma) {
                    None => (
                        &line_ref[..(line_ref.len() - length_newline(line_ref))],
                        None,
                    ),
                    Some(i) => (&line_ref[..i], Some(i)),
                };
                // Check to make sure a quote does not appear in field.
                if !self.lazy_quotes {
                    if let Some(j) = field.find('"') {
                        let err = ParseError::new(
                            rec_line,
                            self.num_line,
                            pos.col + j,
                            ReadError::BareQuote,
                        );
                        break 'parse_field Some(err);
                    }
                }
                self.record_buffer.extend_from_slice(field.as_bytes());
                self.field_indices.push(self.record_buffer.len());
                self.field_positions.push(pos);
                if let Some(i) = i {
                    line_ref = &line_ref[(i + comma_len)..];
                    pos.col += i + comma_len;
                    continue 'parse_field;
                }
                break 'parse_field None;
            } else {
                // Quoted string field
                let field_pos = pos;
                line_ref = &line_ref[QUOTE_LEN..];
                pos.col += QUOTE_LEN;
                loop {
                    match line_ref.find('"') {
                        Some(i) => {
                            // hit next quote
                            self.record_buffer
                                .extend_from_slice(line_ref[..i].as_bytes());
                            line_ref = &line_ref[(i + QUOTE_LEN)..];
                            pos.col += i + QUOTE_LEN;
                            let rn = line_ref.chars().next();
                            if rn == Some('"') {
                                // `""` sequence (append quote).
                                self.record_buffer.push(b'"');
                                line_ref = &line_ref[QUOTE_LEN..];
                                pos.col += QUOTE_LEN;
                            } else if rn == Some(self.comma) {
                                // `",` sequence (end of field).
                                line_ref = &line_ref[comma_len..];
                                pos.col += comma_len;
                                self.field_indices.push(self.record_buffer.len());
                                self.field_positions.push(field_pos);
                                continue 'parse_field;
                            } else if length_newline(line_ref) == line_ref.len() {
                                // `"\n` sequence (end of line).
                                self.field_indices.push(self.record_buffer.len());
                                self.field_positions.push(field_pos);
                                break 'parse_field None;
                            } else if self.lazy_quotes {
                                // `"` sequence (bare quote).
                                self.record_buffer.push(b'"');
                            } else {
                                // `"*` sequence (invalid non-escaped quote).
                                let err = ParseError::new(
                                    rec_line,
                                    self.num_line,
                                    pos.col - QUOTE_LEN,
                                    ReadError::Quote,
                                );
                                break 'parse_field Some(err);
                            }
                        }
                        None if !line_ref.is_empty() => {
                            // Hit end of line (copy all data so far).
                            self.record_buffer.extend_from_slice(line_ref.as_bytes());

                            if err_read.is_some() {
                                break 'parse_field None;
                            }
                            pos.col += line_ref.len();

                            // clear buf
                            line.clear();
                            line_ref = line.as_str();

                            match self.read_line() {
                                Ok(v) => {
                                    line = v;
                                    line_ref = line.as_str();
                                    pos.line += 1;
                                    pos.col = 1;
                                }
                                Err(ReadError::Eof) => err_read = None,
                                Err(err) => err_read = Some(err),
                            }
                        }
                        _ => {
                            // Abrupt end of file (EOF or error).
                            if !self.lazy_quotes && err_read.is_none() {
                                let err =
                                    ParseError::new(rec_line, pos.line, pos.col, ReadError::Quote);
                                break 'parse_field Some(err);
                            }
                            self.field_indices.push(self.record_buffer.len());
                            self.field_positions.push(field_pos);
                            break 'parse_field None;
                        }
                    }
                }
            }
        };
        if let Some(v) = err {
            return Err((v, dst));
        } else if let Some(v) = err_read {
            return Err((v.into(), dst));
        }

        let (mut out, reused) = match dst {
            Some(mut v) => {
                if v.capacity() < self.field_indices.len() {
                    v.reserve_exact(self.field_indices.len() - v.capacity());
                }
                (v, true)
            }
            None => (Vec::with_capacity(self.field_indices.len()), false),
        };
        out.clear();

        let mut prev_idx = 0usize;
        for &v in &self.field_indices {
            let s = String::from_utf8_lossy(&self.record_buffer[prev_idx..v]).to_string();
            out.push(s);
            prev_idx = v;
        }

        // Check or update the expected fields per record.
        match self.fields_per_record {
            None => {}
            Some(0) => self.fields_per_record = Some(out.len()),
            Some(v) => {
                if out.len() != v {
                    let err = ParseError::new(rec_line, rec_line, 1, ReadError::FieldCount);
                    let buf = if reused { Some(out) } else { None };
                    return Err((err, buf));
                }
            }
        }

        Ok(out)
    }
}

/// Returns a new [Reader] that reads from r.
pub fn new_reader<R>(r: R) -> Reader<R>
where
    R: Read,
{
    Reader::new(r)
}

/// reports the number of bytes for the trailing \n.
fn length_newline<S>(b: S) -> usize
where
    S: AsRef<[u8]>,
{
    let b = b.as_ref();

    if (b.len() > 0) && (b[b.len() - 1] == b'\n') {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests;

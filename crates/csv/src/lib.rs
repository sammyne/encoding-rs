//! Crate csv reads and writes comma-separated values (CSV) files.
//! There are many kinds of CSV files; this package supports the format
//! described in [RFC 4180].
//!
//! A csv file contains zero or more records of one or more fields per record.
//! Each record is separated by the newline character. The final record may
//! optionally be followed by a newline character.
//!
//! ```text
//!	field1,field2,field3
//! ```
//!
//! White space is considered part of a field.
//!
//! Carriage returns before newline characters are silently removed.
//!
//! Blank lines are ignored. A line with only whitespace characters (excluding
//! the ending newline character) is not considered a blank line.
//!
//! Fields which start and stop with the quote character `"` are called
//! quoted-fields. The beginning and ending quote are not part of the
//! field.
//!
//! The source:
//!
//! ```text
//!	normal string,"quoted-field"
//! ```
//!
//! results in the fields
//!
//! ```text
//!	{`normal string`, `quoted-field`}
//! ```
//!
//! Within a quoted-field a quote character followed by a second quote
//! character is considered a single quote.
//!
//! ```text
//!	"the ""word"" is true","a ""quoted-field"""
//! ```
//!
//! results in
//!
//! ```text
//!	{`the "word" is true`, `a "quoted-field"`}
//! ```
//!
//! Newlines and commas may be included in a quoted-field
//!
//! ```text
//!	"Multi-line
//!	field","comma is ,"
//! ```
//!
//! results in
//!
//! ```text
//!	{`Multi-line
//!	field`, `comma is ,`}
//! ```
//! 
//! [RFC 4180]: https://rfc-editor.org/rfc/rfc4180.html

mod reader;
mod writer;

pub(crate) mod validator;

pub use reader::*;
pub use writer::*;

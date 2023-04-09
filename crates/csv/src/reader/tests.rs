use std::collections::HashMap;
use std::io::Cursor;

use lazy_static::lazy_static;

use super::{Error, ParseError, Reader};

#[test]
fn read() {
    for tt in READ_TESTS.iter() {
        let test_name = tt.name;
        println!("{}", test_name);
        //if test_name != "BadComma1" {
        //    continue;
        //}

        let (mut r, positions, err_positions, input) = new_reader(tt);

        let out = r.read_all();
        match first_error(&tt.errors, &positions, &err_positions) {
            Some(want_err) => match &out {
                Err(err) => assert!(
                    want_err.equal_partially(err),
                    "[{test_name}] read_all() error mismatch:\ngot  {err}\nwant {want_err:?}"
                ),
                Ok(v) => panic!("[{test_name}] read_all() output:\n got {v:?}\nwant nil"),
            },
            None => match &out {
                Err(err) => panic!("[{test_name}] unexpected read_all() error: {err}"),
                Ok(v) => assert_eq!(v.clone(), tt.output, "[{test_name}] read_all output",),
            },
        };

        // Check input offset after call read_all()
        let input_byte_size = input.len();
        let input_offset = r.input_offset();
        if out.is_ok() {
            assert_eq!(
                input_byte_size, input_offset,
                "[{test_name}] wrong input offset after call read_all():\ninput: {input}"
            );
        }

        // Check field and error positions.
        let (mut r, _, _, _) = new_reader(tt);
        for rec_num in 0.. {
            let want_err = if (rec_num < tt.errors.len()) && tt.errors[rec_num].is_some() {
                Some(error_with_position(
                    tt.errors[rec_num].as_ref().unwrap(),
                    rec_num,
                    &positions,
                    &err_positions,
                ))
            } else if rec_num >= tt.output.len() {
                Some(ReadError::Parse(ParseError::new(0, 0, 0, Error::Eof)))
            } else {
                None
            };

            match r.read() {
                Ok(rec) => {
                    assert!(
                        want_err.is_none(),
                        "[{test_name}] read() unexpected error at record {rec_num}:\ngot {:?}",
                        want_err.unwrap()
                    );
                    assert_eq!(
                        rec, tt.output[rec_num],
                        "[{test_name}] read vs readall mismatch"
                    );

                    let pos = &positions[rec_num];
                    assert_eq!(
                        pos.len(),
                        rec.len(),
                        "[{test_name}] mismatched position length at record {rec_num}"
                    );
                    for i in 0..rec.len() {
                        let (line, col) = r.field_pos(i);
                        let got = [line, col];
                        assert_eq!(
                            got, pos[i],
                            "[{test_name}] position mismatch at record {rec_num}, field {i}"
                        );
                    }
                }
                Err(err) => {
                    match err.err {
                        Error::FieldCount => {}
                        _ => {
                            assert!(
                                rec_num >= tt.output.len(),
                                "[{test_name}] need more records"
                            );
                            break;
                        }
                    }

                    let expect = want_err.expect(&format!("[{test_name}] miss error"));
                    assert!(
                        expect.equal_partially(&err),
                        "[{test_name}] read() error at record {rec_num}: expect {expect:?}, got {err:?}"
                    );
                }
            }
        }
    }
}

lazy_static! {
    static ref READ_TESTS: Vec<ReadTest> = vec![
        ReadTest::default()
            .with_name("Simple")
            .with_input("§a,§b,§c\n")
            .with_output(&[&["a", "b", "c"]]),
        ReadTest::default()
            .with_name("CRLF")
            .with_input("§a,§b\r\n¶§c,§d\r\n")
            .with_output(&[&["a","b"],&["c","d"]]),
        ReadTest::default()
            .with_name("BareCR")
            .with_input("§a,§b\rc,§d\r\n")
            .with_output(&[&["a","b\rc","d"]]),
        ReadTest::default()
            .with_name("RFC4180test")
            .with_fields_per_record(Some(0))
            .with_input(r#"§#field1,§field2,§field3
¶§"aaa",§"bb
b",§"ccc"
¶§"a,a",§"b""bb",§"ccc"
¶§zzz,§yyy,§xxx
"#)
            .with_output(&[&["#field1","field2","field3"],&["aaa","bb\nb","ccc"],&["a,a","b\"bb","ccc"],&["zzz","yyy","xxx"]]),
        ReadTest::default()
            .with_name("NoEOLTest")
            .with_input("§a,§b,§c")
            .with_output(&[&["a","b","c"]]),
        ReadTest::default()
            .with_comma(';')
            .with_name("Semicolon")
            .with_input("§a;§b;§c\n")
            .with_output(&[&["a","b","c"]]),
        ReadTest::default()
            .with_name("MultiLine")
            .with_input("§\"two\nline\",§\"one line\",§\"three\nline\nfield\"")
            .with_output(&[&["two\nline","one line","three\nline\nfield"]]),
        ReadTest::default()
            .with_name("BlankLine")
            .with_input("§a,§b,§c\n\n¶§d,§e,§f\n\n")
            .with_output(&[&["a","b","c"],&["d","e","f"]]),
        ReadTest::default()
            .with_name("BlankLineFieldCount")
            .with_fields_per_record(Some(0))
            .with_input("§a,§b,§c\n\n¶§d,§e,§f\n\n")
            .with_output(&[&["a","b","c"],&["d","e","f"]]),
        ReadTest::default()
            .with_name("TrimSpace")
            .with_input(" §a,  §b,   §c\n")
            .with_output(&[&["a","b","c"]])
            .with_trim_leading_space(true),
        ReadTest::default()
            .with_name("LeadingSpace")
            .with_input("§ a,§  b,§   c\n")
            .with_output(&[&[" a","  b","   c"]]),
        ReadTest::default()
            .with_comment('#')
            .with_name("Comment")
            .with_input("#1,2,3\n§a,§b,§c\n#comment")
            .with_output(&[&["a","b","c"]]),
        ReadTest::default()
            .with_name("NoComment")
            .with_input("§#1,§2,§3\n¶§a,§b,§c")
            .with_output(&[&["#1","2","3"],&["a","b","c"]]),
        ReadTest::default()
            .with_name("LazyQuotes")
            .with_input(r#"§a "word",§"1"2",§a",§"b"#)
            .with_lazy_quotes(true)
            .with_output(&[&[r#"a "word""#, r#"1"2"#, r#"a""#, "b"]]),
        ReadTest::default()
            .with_name("BareQuotes")
            .with_input(r#"§a "word",§"1"2",§a""#)
            .with_lazy_quotes(true)
            .with_output(&[&[r#"a "word""#, r#"1"2"#, r#"a""#]]),
        ReadTest::default()
            .with_name("BareDoubleQuotes")
            .with_input(r#"§a""b,§c"#)
            .with_lazy_quotes(true)
            .with_output(&[&[r#"a""b"#, "c"]]),
        ReadTest::default()
            .with_name("BadDoubleQuotes")
            .with_errors([Some(Error::BareQuote.into())])
            .with_input(r#"§a∑""b,c"#),
        ReadTest::default()
            .with_name("TrimQuote")
            .with_input(r#" §"a",§" b",§c"#)
            .with_trim_leading_space(true)
            .with_output(&[&["a"," b","c"]]),
        ReadTest::default()
            .with_name("BadBareQuote")
            .with_errors([Some(Error::BareQuote.into())])
            .with_input(r#"§a ∑"word","b""#),
        ReadTest::default()
            .with_name("BadTrailingQuote")
            .with_errors([Some(Error::BareQuote.into())])
            .with_input(r#"§"a word",b∑""#),
        ReadTest::default()
            .with_name("ExtraneousQuote")
            .with_errors([Some(Error::Quote.into())])
            .with_input(r#"§"a ∑"word","b""#),
        ReadTest::default()
            .with_name("BadFieldCount")
            .with_errors([None, Some(Error::FieldCount.into())])
            .with_fields_per_record(Some(0))
            .with_input("§a,§b,§c\n¶∑§d,§e")
            .with_output(&[&["a","b","c"], &["d","e"]]),
        ReadTest::default()
            .with_name("BadFieldCountMultiple")
            .with_fields_per_record(Some(0))
            .with_input("§a,§b,§c\n¶∑§d,§e\n¶∑§f")
            .with_errors([None, Some(Error::FieldCount.into()), Some(Error::FieldCount.into())])
            .with_output(&[&["a","b","c"], &["d","e"], &["f"]]),
        ReadTest::default()
            .with_name("BadFieldCount1")
            .with_errors([Some(Error::FieldCount.into())])
            .with_fields_per_record(Some(2))
            .with_input("§∑a,§b,§c")
            .with_output(&[&["a","b","c"]]),
        ReadTest::default()
            .with_name("FieldCount")
            .with_input("§a,§b,§c\n¶§d,§e")
            .with_output(&[&["a","b","c"], &["d","e"]]),
        ReadTest::default()
            .with_name("TrailingCommaEOF")
            .with_input("§a,§b,§c,§")
            .with_output(&[&["a","b","c",""]]),
        ReadTest::default()
            .with_name("TrailingCommaEOL")
            .with_input("§a,§b,§c,§\n")
            .with_output(&[&["a","b","c",""]]),
        ReadTest::default()
            .with_name("TrailingCommaSpaceEOF")
            .with_input("§a,§b,§c, §")
            .with_output(&[&["a","b","c",""]])
            .with_trim_leading_space(true),
        ReadTest::default()
            .with_name("TrailingCommaSpaceEOL")
            .with_input("§a,§b,§c, §\n")
            .with_output(&[&["a","b","c",""]])
            .with_trim_leading_space(true),
        ReadTest::default()
            .with_name("TrailingCommaLine3")
            .with_input("§a,§b,§c\n¶§d,§e,§f\n¶§g,§hi,§")
            .with_output(&[&["a","b","c"],&["d","e","f"],&["g","hi",""]])
            .with_trim_leading_space(true),
        ReadTest::default()
            .with_name("NotTrailingComma3")
            .with_input("§a,§b,§c,§ \n")
            .with_output(&[&["a","b","c"," "]]),
        ReadTest::default()
            .with_name("CommaFieldTest")
            .with_input(r#"§x,§y,§z,§w
¶§x,§y,§z,§
¶§x,§y,§,§
¶§x,§,§,§
¶§,§,§,§
¶§"x",§"y",§"z",§"w"
¶§"x",§"y",§"z",§""
¶§"x",§"y",§"",§""
¶§"x",§"",§"",§""
¶§"",§"",§"",§""
"#)
            .with_output(&[
                &["x","y","z","w"],
                &["x","y","z",""],
                &["x","y","",""],
                &["x","","",""],
                &["","","",""],
                &["x","y","z","w"],
                &["x","y","z",""],
                &["x","y","",""],
                &["x","","",""],
                &["","","",""]
            ]),
        ReadTest::default()
            .with_name("TrailingCommaIneffective1")
            .with_input("§a,§b,§\n¶§c,§d,§e")
            .with_output(&[&["a","b",""],&["c","d","e"]])
            .with_trim_leading_space(true),
        ReadTest::default()
            .with_name("ReadAllReuseRecord")
            .with_input("§a,§b\n¶§c,§d")
            .with_output(&[&["a","b"],&["c","d"]]),
        ReadTest::default()
            .with_name("StartLine1")
            .with_errors([Some(Error::Quote.into())])
            .with_input("§a,\"b\nc∑\"d,e"),
        ReadTest::default()
            .with_name("StartLine2")
            .with_errors([None, Some(Error::Quote.into())])
            .with_input("§a,§b\n¶§\"d\n\n,e∑")
            .with_output(&[&["a","b"]]),
        ReadTest::default()
            .with_name("CRLFInQuotedField")
            .with_input("§A,§\"Hello\r\nHi\",§B\r\n")
            .with_output(&[&["A","Hello\nHi","B"]]),
        ReadTest::default()
            .with_name("BinaryBlobField")
            .with_input("§x09A\u{b41c},§aktau")
            .with_output(&[&["x09A\u{b41c}","aktau"]]),
        ReadTest::default()
            .with_name("TrailingCR")
            .with_input("§field1,§field2\r")
            .with_output(&[&["field1","field2"]]),
        ReadTest::default()
            .with_name("QuotedTrailingCR")
            .with_input("§\"field\"\r")
            .with_output(&[&["field"]]),
        ReadTest::default()
            .with_name("QuotedTrailingCRCR")
            .with_errors([Some(Error::Quote.into())])
            .with_input("§\"field∑\"\r\r"),
        ReadTest::default()
            .with_name("FieldCR")
            .with_input("§field\rfield\r")
            .with_output(&[&["field\rfield"]]),
        ReadTest::default()
            .with_name("FieldCRCR")
            .with_input("§field\r\rfield\r\r")
            .with_output(&[&["field\r\rfield\r"]]),
        ReadTest::default()
            .with_name("FieldCRCRLF")
            .with_input("§field\r\r\n¶§field\r\r\n")
            .with_output(&[&["field\r"],&["field\r"]]),
        ReadTest::default()
            .with_name("FieldCRCRLFCR")
            .with_input("§field\r\r\n¶§\rfield\r\r\n\r")
            .with_output(&[&["field\r"],&["\rfield\r"]]),
        ReadTest::default()
            .with_name("FieldCRCRLFCRCR")
            .with_input("§field\r\r\n¶§\r\rfield\r\r\n¶§\r\r")
            .with_output(&[&["field\r"],&["\r\rfield\r"],&["\r"]]),
        ReadTest::default()
            .with_name("MultiFieldCRCRLFCRCR")
            .with_input("§field1,§field2\r\r\n¶§\r\rfield1,§field2\r\r\n¶§\r\r,§")
            .with_output(&[&["field1","field2\r"],&["\r\rfield1","field2\r"],&["\r\r",""]]),
        ReadTest::default()
            .with_name("NonASCIICommaAndComment")
            .with_comma('£')
            .with_comment('€')
            .with_input("§a£§b,c£ \t§d,e\n€ comment\n")
            .with_input("§a£§b,c£ \t§d,e\n€ comment\n")
            .with_input("§a£§b,c£ \t§d,e\n€ comment\n")
            .with_output(&[&["a","b,c","d,e"]])
            .with_trim_leading_space(true),
        ReadTest::default()
            .with_name("NonASCIICommaAndCommentWithQuotes")
            .with_comma('€')
            .with_comment('λ')
            .with_input("§a€§\"  b,\"€§ c\nλ comment\n")
            .with_output(&[&["a","  b,"," c"]]),
        ReadTest::default()
            .with_name("NonASCIICommaConfusion")
            .with_comma('λ')
            .with_comment('€')
            .with_input("§\"abθcd\"λ§efθgh")
            .with_output(&[&["abθcd","efθgh"]]),
        ReadTest::default()
            .with_name("NonASCIICommentConfusion")
            .with_comment('θ')
            .with_input("§λ\n¶§λ\nθ\n¶§λ\n")
            .with_output(&[&["λ"],&["λ"],&["λ"]]),
        ReadTest::default()
            .with_name("QuotedFieldMultipleLF")
            .with_input("§\"\n\n\n\n\"")
            .with_output(&[&["\n\n\n\n"]]),
        ReadTest::default()
            .with_name("MultipleCRLF")
            .with_input("\r\n\r\n\r\n\r\n"),
        ReadTest::default()
            .with_name("HugeLines")
            .with_comment('#')
            .with_input("#ignore\n".repeat(10000) + "§" + &"@".repeat(5000) + ",§" + &"*".repeat(5000))
            .with_output(&[&["@".repeat(5000), "*".repeat(5000)]]),
        ReadTest::default()
            .with_name("QuoteWithTrailingCRLF")
            .with_errors([Some(Error::Quote.into())])
            .with_input("§\"foo∑\"bar\"\r\n"),
        ReadTest::default()
            .with_name("LazyQuoteWithTrailingCRLF")
            .with_input("§\"foo\"bar\"\r\n")
            .with_lazy_quotes(true)
            .with_output(&[&["foo\"bar"]]),
        ReadTest::default()
            .with_name("DoubleQuoteWithTrailingCRLF")
            .with_input("§\"foo\"\"bar\"\r\n")
            .with_output(&[&["foo\"bar"]]),
        ReadTest::default()
            .with_name("EvenQuotes")
            .with_input(r#"§"""""""""#)
            .with_output(&[&[r#"""""#]]),
        ReadTest::default()
            .with_name("OddQuotes")
            .with_errors([Some(Error::Quote.into())])
            .with_input(r#"§"""""""∑"#),
        ReadTest::default()
            .with_name("LazyOddQuotes")
            .with_input(r#"§""""""""#)
            .with_lazy_quotes(true)
            .with_output(&[&[r#"""""#]]),
        ReadTest::default()
            .with_name("BadComma1")
            .with_comma('\n')
            .with_other_errors([Some(Error::InvalidDelimiter)]),
        ReadTest::default()
            .with_name("BadComma2")
            .with_comma('\r')
            .with_other_errors([Some(Error::InvalidDelimiter)]),
        ReadTest::default()
            .with_name("BadComma3")
            .with_comma('"')
            .with_other_errors([Some(Error::InvalidDelimiter)]),
        ReadTest::default()
            .with_name("BadComma4")
            .with_comma('�')
            .with_other_errors([Some(Error::InvalidDelimiter)]),
        ReadTest::default()
            .with_name("BadComment1")
            .with_comment('\n')
            .with_other_errors([Some(Error::InvalidDelimiter)]),
        ReadTest::default()
            .with_name("BadComment2")
            .with_comment('\r')
            .with_other_errors([Some(Error::InvalidDelimiter)]),
        ReadTest::default()
            .with_name("BadComment3")
            .with_comment('�')
            .with_other_errors([Some(Error::InvalidDelimiter)]),
        ReadTest::default()
            .with_name("BadCommaComment")
            .with_comma('X')
            .with_comment('X')
            .with_other_errors([Some(Error::InvalidDelimiter)]),
    ];
}

#[derive(Debug)]
enum ReadError {
    Parse(ParseError),
    Other(Error),
}

#[derive(Default)]
struct ReadTest {
    name: &'static str,
    input: String,
    output: Vec<Vec<String>>,
    // positions: Vec<Vec<[usize; 2]>>,
    errors: Vec<Option<ReadError>>,

    comma: Option<char>,
    comment: Option<char>,
    fields_per_record: Option<usize>,
    lazy_quotes: bool,
    trim_leading_space: bool,
}

impl ReadError {
    fn equal_partially(&self, err: &ParseError) -> bool {
        match self {
            ReadError::Parse(v) => v.equal_partially(err),
            ReadError::Other(v) => v.equal_partially(&err.err),
        }
    }
}

impl ReadTest {
    fn with_comma(mut self, comma: char) -> Self {
        self.comma = Some(comma);
        self
    }

    fn with_comment(mut self, comment: char) -> Self {
        self.comment = Some(comment);
        self
    }

    fn with_errors<const N: usize>(mut self, errors: [Option<ParseError>; N]) -> Self {
        self.errors = errors
            .into_iter()
            .map(|v| v.map(ReadError::Parse))
            .collect();
        self
    }

    fn with_other_errors<const N: usize>(mut self, errors: [Option<Error>; N]) -> Self {
        self.errors = errors
            .into_iter()
            .map(|v| v.map(ReadError::Other))
            .collect();
        self
    }

    fn with_fields_per_record(mut self, fields_per_record: Option<usize>) -> Self {
        self.fields_per_record = fields_per_record;
        self
    }

    fn with_input<S>(mut self, input: S) -> Self
    where
        S: ToString,
    {
        self.input = input.to_string();
        self
    }

    fn with_lazy_quotes(mut self, lazy_quotes: bool) -> Self {
        self.lazy_quotes = lazy_quotes;
        self
    }

    fn with_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }

    fn with_output<S>(mut self, output: &[&[S]]) -> Self
    where
        S: ToString,
    {
        self.output = output
            .iter()
            .map(|&v| v.iter().map(|w| w.to_string()).collect::<Vec<_>>())
            .collect();
        self
    }

    //fn with_positions(mut self, positions: &[&[&[usize; 2]]]) -> Self {
    //    let mut buf = Vec::with_capacity(positions.len());

    //    for &v in positions {
    //        let b = v.iter().map(|&w| *w).collect();
    //        buf.push(b);
    //    }

    //    self.positions = buf;
    //    self
    //}

    fn with_trim_leading_space(mut self, trim_leading_space: bool) -> Self {
        self.trim_leading_space = trim_leading_space;
        self
    }
}

fn copy_error_partially(err: &Error) -> Error {
    match err {
        Error::BareQuote => Error::BareQuote,
        Error::Eof => Error::Eof,
        Error::FieldCount => Error::FieldCount,
        Error::InvalidDelimiter => Error::InvalidDelimiter,
        Error::Io(v) => Error::Io(v.kind().into()),
        Error::Quote => Error::Quote,
        Error::TrailingComma => Error::TrailingComma,
    }
}

fn error_with_position(
    err: &ReadError,
    rec_num: usize,
    positions: &[Vec<[usize; 2]>],
    err_positions: &HashMap<usize, [usize; 2]>,
) -> ReadError {
    match err {
        ReadError::Other(v) => ReadError::Other(copy_error_partially(v)),
        ReadError::Parse(v) => {
            assert!(
                rec_num < positions.len(),
                "no positions found for error at record {}",
                rec_num
            );
            let err_pos = err_positions.get(&rec_num).expect(&format!(
                "no error position found for error at record {}",
                rec_num
            ));

            let err = copy_error_partially(&v.err);
            let parse_err = ParseError::new(positions[rec_num][0][0], err_pos[0], err_pos[1], err);

            ReadError::Parse(parse_err)
        }
    }
}

/// firstError returns the first non-nil error in errs,
/// with the position adjusted according to the error's
/// index inside positions.
fn first_error(
    errs: &[Option<ReadError>],
    positions: &[Vec<[usize; 2]>],
    err_positions: &HashMap<usize, [usize; 2]>,
) -> Option<ReadError> {
    for (i, v) in errs.iter().enumerate() {
        if let Some(err) = v {
            return Some(error_with_position(err, i, positions, err_positions));
        }
    }

    None
}

/// makePositions returns the expected field positions of all
/// the fields in text, the positions of any errors, and the text with the position markers
/// removed.
///
/// The start of each field is marked with a § symbol;
/// CSV lines are separated by ¶ symbols;
/// Error positions are marked with ∑ symbols.
fn make_positions(text: &str) -> (Vec<Vec<[usize; 2]>>, HashMap<usize, [usize; 2]>, String) {
    let mut buf = Vec::with_capacity(text.len());
    let mut positions = vec![];
    let mut err_positions = HashMap::new();
    let (mut line, mut col) = (1, 1);
    let mut rec_num = 0usize;

    let mut cbuf = [0u8; 4];
    for (_, r) in text.char_indices() {
        match r {
            '\n' => {
                line += 1;
                col = 1;
                buf.push(b'\n');
            }
            '§' => {
                if positions.is_empty() {
                    positions.push(vec![]);
                }
                positions.last_mut().unwrap().push([line, col]);
            }
            '¶' => {
                positions.push(vec![]);
                rec_num += 1;
            }
            '∑' => {
                err_positions.insert(rec_num, [line, col]);
                //col += r.len_utf8();
            }
            _ => {
                buf.extend_from_slice(r.encode_utf8(&mut cbuf).as_bytes());
                col += r.len_utf8();
            }
        }
    }

    let b = unsafe { String::from_utf8_unchecked(buf) };
    (positions, err_positions, b)
}

fn new_reader(
    tt: &ReadTest,
) -> (
    Reader<Cursor<Vec<u8>>>,
    Vec<Vec<[usize; 2]>>,
    HashMap<usize, [usize; 2]>,
    String,
) {
    let (positions, err_positions, input) = make_positions(&tt.input);
    let mut r = Reader::new(Cursor::new(input.as_bytes().to_vec()));

    if let Some(c) = tt.comma {
        r.comma = c;
    }
    r.comment = tt.comment;
    r.fields_per_record = tt.fields_per_record;
    r.lazy_quotes = tt.lazy_quotes;
    r.trim_leading_space = tt.trim_leading_space;
    (r, positions, err_positions, input)
}

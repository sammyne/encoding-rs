use std::io;

use lazy_static::lazy_static;

use crate::writer::{self, WriteError};

#[test]
fn error() {
    let mut b = Vec::new();
    let mut f = writer::new_writer(&mut b);
    let _ = f.write(["abc"]);
    let _ = f.flush();

    assert!(f.error().is_none(), "unexpected error");

    let mut f = writer::new_writer(ErrorWriter);
    let _ = f.write(["abc"]);
    let _ = f.flush();

    assert!(f.error().is_some(), "miss error");
}

#[test]
fn write() {
    for (n, tt) in WRITE_TESTS.iter().enumerate() {
        let mut b = Vec::new();

        let r = {
            let mut f = writer::new_writer(&mut b);
            f.use_crlf = tt.use_crlf;
            if let Some(c) = tt.comma {
                f.comma = c;
            }

            f.write_all(&tt.input)
        };

        match r {
            Ok(_) => {
                assert!(tt.error.is_none(), "expect error");
                let got = unsafe { String::from_utf8_unchecked(b) };
                assert_eq!(tt.output, got, "#{n}");
            }
            Err(err) => assert!(
                err.equal_partially(tt.error.as_ref().unwrap()),
                "bad error: expect {:?}, got {:?}",
                err,
                tt.output
            ),
        }
    }
}

struct ErrorWriter;

#[derive(Default)]
struct WriteTest {
    input: Vec<Vec<&'static str>>,
    output: &'static str,
    error: Option<WriteError>,
    use_crlf: bool,
    comma: Option<char>,
}

lazy_static! {
    static ref WRITE_TESTS: Vec<WriteTest> = vec![
        WriteTest {
            input: vec![vec!["abc"]],
            output: "abc\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["abc"]],
            output: "abc\r\n",
            use_crlf: true,
            ..Default::default()
        },
        WriteTest {
            input: vec![vec![r#""abc""#]],
            output: std::concat!(r#""""abc""""#, '\n'),
            ..Default::default()
        },
        WriteTest {
            input: vec![vec![r#"a"b"#]],
            output: std::concat!(r#""a""b""#, '\n'),
            ..Default::default()
        },
        WriteTest {
            input: vec![vec![r#""a"b""#]],
            output: std::concat!(r#""""a""b""""#, '\n'),
            ..Default::default()
        },
        WriteTest {
            input: vec![vec![" abc"]],
            output: std::concat!(r#"" abc""#, '\n'),
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["abc,def"]],
            output: std::concat!(r#""abc,def""#, '\n'),
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["abc", "def"]],
            output: "abc,def\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["abc"], vec!["def"]],
            output: "abc\ndef\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["abc\ndef"]],
            output: "\"abc\ndef\"\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["abc\ndef"]],
            output: "\"abc\r\ndef\"\r\n",
            use_crlf: true,
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["abc\rdef"]],
            output: "\"abcdef\"\r\n",
            use_crlf: true,
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["abc\rdef"]],
            output: "\"abc\rdef\"\n",
            use_crlf: false,
            ..Default::default()
        },
        WriteTest {
            input: vec![vec![""]],
            output: "\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["", ""]],
            output: ",\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["", "", ""]],
            output: ",,\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["", "", "a"]],
            output: ",,a\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["", "a", ""]],
            output: ",a,\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["", "a", "a"]],
            output: ",a,a\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["a", "", ""]],
            output: "a,,\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["a", "", "a"]],
            output: "a,,a\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["a", "a", ""]],
            output: "a,a,\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["a", "a", "a"]],
            output: "a,a,a\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec![r#"\."#]],
            output: "\"\\.\"\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["x09\u{41b4}\x1c", "aktau"]],
            output: "x09\u{41b4}\x1c,aktau\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec![",x09\u{41b4}\x1c", "aktau"]],
            output: "\",x09\u{41b4}\x1c\",aktau\n",
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["a", "a", ""]],
            output: "a|a|\n",
            comma: Some('|'),
            ..Default::default()
        },
        WriteTest {
            input: vec![vec![",", ",", ""]],
            output: ",|,|\n",
            comma: Some('|'),
            ..Default::default()
        },
        WriteTest {
            input: vec![vec!["foo"]],
            comma: Some('"'),
            error: Some(WriteError::InvalidDelimiter),
            ..Default::default()
        },
    ];
}

impl io::Write for ErrorWriter {
    fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::ErrorKind::Other.into())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl WriteError {
    fn equal_partially(&self, other: &Self) -> bool {
        match (self, other) {
            (WriteError::InvalidDelimiter, WriteError::InvalidDelimiter) => true,
            (WriteError::Io(a), WriteError::Io(b)) => a.kind() == b.kind(),
            (_, _) => false,
        }
    }
}

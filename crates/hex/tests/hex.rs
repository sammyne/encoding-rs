use std::io::{self, Read, Write};

use lazy_static::lazy_static;

use hex::{self, Error};

const EXPECTED_HEX_DUMP: &'static str = r##"00000000  1e 1f 20 21 22 23 24 25  26 27 28 29 2a 2b 2c 2d  |.. !"#$%&'()*+,-|
00000010  2e 2f 30 31 32 33 34 35  36 37 38 39 3a 3b 3c 3d  |./0123456789:;<=|
00000020  3e 3f 40 41 42 43 44 45                           |>?@ABCDE|
"##;

fn encode_decode_test_cases() -> Vec<(String, Vec<u8>)> {
    vec![
        (String::from(""), vec![]),
        (
            String::from("0001020304050607"),
            vec![0, 1, 2, 3, 4, 5, 6, 7],
        ),
        (
            String::from("08090a0b0c0d0e0f"),
            vec![8, 9, 10, 11, 12, 13, 14, 15],
        ),
        (
            String::from("f0f1f2f3f4f5f6f7"),
            vec![0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7],
        ),
        (
            String::from("f8f9fafbfcfdfeff"),
            vec![0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff],
        ),
        (String::from("67"), vec![b'g']),
        (String::from("e3a1"), vec![0xe3, 0xa1]),
    ]
}

#[test]
fn dumper() {
    let data = {
        let mut v = [0u8; 40];
        for i in 0..v.len() {
            v[i] = (i + 30) as u8;
        }
        v
    };

    for stride in 1..data.len() {
        let mut out = Vec::new();
        let mut dumper = hex::dumper(&mut out);

        let mut done = 0usize;

        while done < data.len() {
            let mut todo = done + stride;
            if todo > data.len() {
                todo = data.len();
            }

            let _ = dumper.write(&data[done..todo]);
            done = todo;
        }

        let _ = dumper.flush();
        std::mem::drop(dumper);

        let got = String::from_utf8(out).expect("nonn-utf8 string");
        assert_eq!(EXPECTED_HEX_DUMP, &got);
    }
}

#[test]
fn dump() {
    let data = {
        let mut v = [0u8; 40];
        for i in 0..v.len() {
            v[i] = (i + 30) as u8;
        }
        v
    };

    let got = hex::dump(&data[..]);

    assert_eq!(EXPECTED_HEX_DUMP, got);
}

#[test]
fn dump_doubleclose() {
    let mut out = Vec::new();
    let mut dumper = hex::dumper(&mut out);

    let _ = dumper.write(b"gopher");
    let _ = dumper.flush();
    let _ = dumper.flush();

    let _ = dumper.write(b"gopher");
    let _ = dumper.flush();
    std::mem::drop(dumper);

    let expected = "00000000  67 6f 70 68 65 72                                 |gopher|\n";
    let got = String::from_utf8(out).expect("invalid utf8 string");
    assert_eq!(expected, got);
}

#[test]
fn dump_earlyclose() {
    let mut out = Vec::new();
    let mut dumper = hex::dumper(&mut out);

    let _ = dumper.flush();
    let _ = dumper.write(b"gopher");
    std::mem::drop(dumper);

    assert_eq!(out.len(), 0, "should write no data");
}

#[test]
fn decode_string() {
    let cases = encode_decode_test_cases();
    for v in cases.iter() {
        let (src, expect) = v;

        let got = hex::decode_string(src).unwrap();
        assert_eq!(expect, &got);
    }
}

#[test]
fn decoder_err() {
    for tt in ERR_TESTS.iter() {
        let mut dec = hex::new_decoder(tt.input.as_bytes());

        let mut out = vec![];
        match dec.read_to_end(&mut out) {
            Ok(_) => {
                assert!(
                    tt.err.is_none(),
                    "input='{}': unexpected error {:?}",
                    tt.input,
                    tt.err
                );
                assert_eq!(
                    tt.output,
                    out.as_slice(),
                    "input='{}': bad output",
                    tt.input
                );
            }
            Err(err) => {
                let (expect, expect_kind) = match tt.err.as_ref().expect("miss error") {
                    Error::Length => (Error::Length, io::ErrorKind::UnexpectedEof),
                    v => (v.clone(), io::ErrorKind::Other),
                };

                assert!(
                    err.kind() == expect_kind,
                    "input='{}': bad error kind, expect {:?}, got {:?}",
                    tt.input,
                    expect_kind,
                    err.kind()
                );

                if err.kind() != io::ErrorKind::Other {
                    continue;
                }

                let got = err
                    .into_inner()
                    .expect("unwrap error")
                    .downcast::<Error>()
                    .expect("unable downcast to Error");
                match (expect, got.as_ref()) {
                    (Error::Length, Error::Length) => {}
                    (Error::InvalidByte(x), Error::InvalidByte(y)) if x == *y => {}
                    (a, b) => panic!(
                        "input='{}': bad error, expect {:?}, got {:?}",
                        tt.input, a, b
                    ),
                }
            }
        }
    }
}

#[test]
fn encode_to_string() {
    let cases = encode_decode_test_cases();
    for v in cases.iter() {
        let (expect, src) = v;

        let got = hex::encode_to_string(src);
        assert_eq!(expect, &got);
    }
}

#[test]
fn encoder_decoder() {
    for multiplier in [1, 128, 192] {
        for test in ENC_DEC_TESTS.iter() {
            let dec_str = unsafe { std::str::from_utf8_unchecked(test.dec) };

            let input = test.dec.repeat(multiplier);
            let output = test.enc.repeat(multiplier);

            let mut buf = vec![];

            {
                let mut enc = hex::new_encoder(&mut buf);

                let mut r = input.as_slice();
                let n = io::copy(&mut r, &mut enc).expect("expect no error");
                assert_eq!(
                    input.len(),
                    n as usize,
                    "Encoder::write('{}'*{}) = Ok({})",
                    dec_str,
                    multiplier,
                    n
                );
            }

            let enc_dst = unsafe { std::str::from_utf8_unchecked(&buf) };
            assert_eq!(output, enc_dst, "buf('{}'*{})", dec_str, multiplier);

            let mut dec = hex::new_decoder(buf.as_slice());
            let mut dec_buf = vec![];

            let _ = io::copy(&mut dec, &mut dec_buf).expect("expect no error for decode");
            assert_eq!(
                input.len(),
                dec_buf.len(),
                "Decoder::read('{}'*{})",
                dec_str,
                multiplier
            );
            assert_eq!(input, dec_buf, "dec_buf('{}'*{})", dec_str, multiplier);
        }
    }
}

struct EncDecTest {
    enc: &'static str,
    dec: &'static [u8],
}

struct ErrTest {
    input: &'static str,
    output: &'static [u8],
    err: Option<Error>,
}

impl EncDecTest {
    fn new(enc: &'static str, dec: &'static [u8]) -> Self {
        Self { enc, dec }
    }
}

impl ErrTest {
    fn new(input: &'static str, output: &'static [u8], err: Option<Error>) -> Self {
        Self { input, output, err }
    }
}

lazy_static! {
    static ref ENC_DEC_TESTS: Vec<EncDecTest> = vec![
        EncDecTest::new("", &[]),
        EncDecTest::new("0001020304050607", &[0, 1, 2, 3, 4, 5, 6, 7]),
        EncDecTest::new("08090a0b0c0d0e0f", &[8, 9, 10, 11, 12, 13, 14, 15]),
        EncDecTest::new(
            "f0f1f2f3f4f5f6f7",
            &[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7]
        ),
        EncDecTest::new(
            "f8f9fafbfcfdfeff",
            &[0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff]
        ),
        EncDecTest::new("67", &[b'g']),
        EncDecTest::new("e3a1", &[0xe3, 0xa1]),
    ];
    static ref ERR_TESTS: Vec<ErrTest> = vec![
        ErrTest::new("", b"", None),
        ErrTest::new("0", b"", Some(Error::Length)),
        ErrTest::new("zd4aa", b"", Some(Error::InvalidByte(b'z'))),
        ErrTest::new("d4aaz", b"\xd4\xaa", Some(Error::InvalidByte(b'z'))),
        ErrTest::new("30313", b"01", Some(Error::Length)),
        ErrTest::new("0g", b"", Some(Error::InvalidByte(b'g'))),
        ErrTest::new("00gg", b"\x00", Some(Error::InvalidByte(b'g'))),
        ErrTest::new("0\x01", b"", Some(Error::InvalidByte(b'\x01'))),
        ErrTest::new("ffeed", b"\xff\xee", Some(Error::Length)),
    ];
}

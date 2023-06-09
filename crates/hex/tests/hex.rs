use std::io::Write;

use hex::{self, Dumper};

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
        let mut dumper = Dumper::new(&mut out);

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
    let mut dumper = Dumper::new(&mut out);

    let _ = dumper.write(b"gopher");
    let _ = dumper.flush();
    let _ = dumper.flush();

    let _ = dumper.write(b"gopher");
    let _ = dumper.flush();

    let expected = "00000000  67 6f 70 68 65 72                                 |gopher|\n";
    let got = String::from_utf8(out).expect("invalid utf8 string");
    assert_eq!(expected, got);
}

#[test]
fn dump_earlyclose() {
    let mut out = Vec::new();
    let mut dumper = Dumper::new(&mut out);

    let _ = dumper.flush();
    let _ = dumper.write(b"gopher");

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
fn encode_to_string() {
    let cases = encode_decode_test_cases();
    for v in cases.iter() {
        let (expect, src) = v;

        let got = hex::encode_to_string(src);
        assert_eq!(expect, &got);
    }
}

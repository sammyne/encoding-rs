use base32::STD_ENCODING;

#[test]
fn decode() {
    for (i, v) in testbot::PAIRS.iter().enumerate() {
        let mut got = vec![0u8; STD_ENCODING.decoded_len(v.encoded.len())];
        let n = STD_ENCODING.decode(&mut got, v.encoded.as_bytes()).unwrap();
        got.truncate(n);

        let expect = testbot::escape_ascii_string(v.decoded);
        let got = testbot::escape_ascii_string(&got);

        assert_eq!(expect, got, "#{i}");
    }
}

#[test]
fn decode_corrupt() {
    struct TestCase {
        input: &'static str,
        offset: Option<usize>,
    }
    let new_case = |input, offset: i32| {
        let offset = if -1 == offset {
            None
        } else {
            Some(offset as usize)
        };
        TestCase { input, offset }
    };

    let test_vector = &[
        new_case("", -1),
        new_case("!!!!", 0),
        new_case("x===", 0),
        new_case("AA=A====", 2),
        new_case("AAA=AAAA", 3),
        new_case("MMMMMMMMM", 8),
        new_case("MMMMMM", 0),
        new_case("A=", 1),
        new_case("AA=", 3),
        new_case("AA==", 4),
        new_case("AA===", 5),
        new_case("AAAA=", 5),
        new_case("AAAA==", 6),
        new_case("AAAAA=", 6),
        new_case("AAAAA==", 7),
        new_case("A=======", 1),
        new_case("AA======", -1),
        new_case("AAA=====", 3),
        new_case("AAAA====", -1),
        new_case("AAAAA===", -1),
        new_case("AAAAAA==", 6),
        new_case("AAAAAAA=", -1),
        new_case("AAAAAAAA", -1),
    ];

    for (i, tc) in test_vector.iter().enumerate() {
        let mut dbuf = vec![0u8; STD_ENCODING.decoded_len(tc.input.len())];
        let result = STD_ENCODING.decode(&mut dbuf, tc.input.as_bytes());
        if tc.offset.is_none() {
            assert!(
                result.is_ok(),
                "#{} Decoder wrongly detected corruption in {}",
                i,
                tc.input
            );
            continue;
        }

        let idx = tc.offset.unwrap();
        let got_err = result.unwrap_err();
        assert_eq!(idx, got_err.idx, "#{} wrong corruption in {}", i, tc.input);
    }
}

#[test]
fn decode_string() {
    for (i, v) in testbot::PAIRS.iter().enumerate() {
        let got = STD_ENCODING.decode_string(v.encoded).unwrap();

        let expect = testbot::escape_ascii_string(v.decoded);
        let got = testbot::escape_ascii_string(&got);

        assert_eq!(expect, got, "#{i}");
    }
}

#[test]
fn new_line_characters() {
    fn test_string_encoding(expected: &str, examples: &[&str]) {
        for (i, v) in examples.iter().enumerate() {
            let buf = STD_ENCODING.decode_string(v).unwrap();
            let got = testbot::escape_ascii_string(&buf);
            assert_eq!(expected, got, "#{} decode({})", i, v);
        }
    }

    let examples = &[
        "ON2XEZI=",
        "ON2XEZI=\r",
        "ON2XEZI=\n",
        "ON2XEZI=\r\n",
        "ON2XEZ\r\nI=",
        "ON2X\rEZ\nI=",
        "ON2X\nEZ\rI=",
        "ON2XEZ\nI=",
        "ON2XEZI\n=",
    ];
    test_string_encoding("sure", examples);

    let examples = &["MZXW6YTBOI======", "MZXW6YTBOI=\r\n====="];
    test_string_encoding("foobar", examples);
}

#[test]
fn with_padding() {
    let encodings = vec![
        *STD_ENCODING,
        STD_ENCODING.clone().with_padding(Some(b'-')).clone(),
        STD_ENCODING.clone().with_padding(None).clone(),
    ];

    for (i, enc) in encodings.iter().enumerate() {
        for (j, pair) in testbot::PAIRS.iter().enumerate() {
            let input = pair.decoded;
            let encoded = enc.encode_to_string(input);

            let decoded = enc.decode_string(encoded.as_str()).unwrap();
            let got = testbot::escape_ascii_string(&decoded);

            let expect = testbot::escape_ascii_string(input);

            assert_eq!(expect, got, "#{i}-#{j} unexpected decoded result");
        }
    }
}

#[test]
fn with_wrong_padding() {
    let encoded = STD_ENCODING.encode_to_string("foobar".as_bytes());

    STD_ENCODING
        .clone()
        .with_padding(Some(b'-'))
        .decode_string(encoded.as_str())
        .expect_err("expected error due to bad padding");

    STD_ENCODING
        .clone()
        .with_padding(None)
        .decode_string(encoded.as_str())
        .expect_err("expected error due to no padding");
}

mod testbot;

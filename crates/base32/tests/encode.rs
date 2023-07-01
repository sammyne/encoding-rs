use base32::STD_ENCODING;

#[test]
fn encode() {
    for (i, v) in testbot::PAIRS.iter().enumerate() {
        let got = STD_ENCODING.encode_to_string(v.decoded);
        assert_eq!(
            v.encoded,
            got,
            "#{} encode({})",
            i,
            testbot::escape_ascii_string(v.decoded)
        );
    }
}

#[test]
fn with_custom_padding() {
    for (i, testcase) in testbot::PAIRS.iter().enumerate() {
        let default_padding = STD_ENCODING.encode_to_string(testcase.decoded);
        let custom_padding = STD_ENCODING
            .clone()
            .with_padding(Some(b'@'))
            .encode_to_string(testcase.decoded);

        let expected = default_padding.replace('=', "@");
        assert_eq!(expected, custom_padding, "#{} custom failed", i);
        assert_eq!(testcase.encoded, default_padding, "#{} standard failed", i)
    }
}

#[test]
fn without_padding() {
    for (i, testcase) in testbot::PAIRS.iter().enumerate() {
        let default_padding = STD_ENCODING.encode_to_string(testcase.decoded);
        let custom_padding = STD_ENCODING
            .clone()
            .with_padding(None)
            .encode_to_string(testcase.decoded);

        let expected = default_padding.trim_end_matches('=');

        assert_eq!(expected, custom_padding, "#{} custom failed", i);
        assert_eq!(testcase.encoded, default_padding, "#{} standard failed", i)
    }
}

mod testbot;

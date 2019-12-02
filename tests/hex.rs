use encoding::hex;

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

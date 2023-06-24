fn main() {
    const ENCODED: &'static [u8] = b"SGVsbG8sIHdvcmxkIQ==";

    let mut dst = vec![0u8; base64::STD_ENCODING.decoded_len(ENCODED.len())];

    let n = base64::STD_ENCODING
        .decode(&mut dst, ENCODED)
        .expect("decode");

    let got = &dst[..n];

    const EXPECT: &'static [u8] = b"Hello, world!";

    assert_eq!(EXPECT, got);
}

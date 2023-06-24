fn main() {
    const DATA: &'static [u8] = b"Hello, world!";

    let mut dst = vec![0u8; base64::STD_ENCODING.encoded_len(DATA.len())];
    base64::STD_ENCODING.encode(&mut dst, DATA);

    let got = String::from_utf8_lossy(&dst);

    const EXPECT: &'static str = "SGVsbG8sIHdvcmxkIQ==";

    assert_eq!(EXPECT, got);
}

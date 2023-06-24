fn main() {
    const DATA: &'static [u8] = b"any + old & data";

    let got = base64::STD_ENCODING.encode_to_string(DATA);

    const EXPECT: &'static str = "YW55ICsgb2xkICYgZGF0YQ==";

    assert_eq!(EXPECT, got);
}

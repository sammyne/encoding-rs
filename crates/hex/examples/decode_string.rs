fn main() {
    const S: &'static str = "48656c6c6f20476f7068657221";

    let decoded = hex::decode_string(S).unwrap();

    let got = unsafe { std::str::from_utf8_unchecked(decoded.as_slice()) };

    const EXPECT: &'static str = "Hello Gopher!";

    assert_eq!(EXPECT, got);
}

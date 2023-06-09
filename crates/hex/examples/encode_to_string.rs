fn main() {
    let src = b"Hello";

    let got = hex::encode_to_string(src);

    const EXPECT: &'static str = "48656c6c6f";

    assert_eq!(EXPECT, got);
}

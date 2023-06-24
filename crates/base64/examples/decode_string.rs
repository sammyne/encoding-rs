fn main() {
    const ENCODED: &'static str = "c29tZSBkYXRhIHdpdGggACBhbmQg77u/";

    let got: String = base64::STD_ENCODING
        .decode_string(ENCODED)
        .map(|v| unsafe { String::from_utf8_unchecked(v) })
        .unwrap();

    const EXPECT: &'static str = "some data with \x00 and \u{feff}";
    assert_eq!(EXPECT, got);
}

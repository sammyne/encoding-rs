fn main() {
    const ENCODED: &str = "ONXW2ZJAMRQXIYJAO5UXI2BAAAQGC3TEEDX3XPY=";

    let got = base32::STD_ENCODING
        .decode_string(ENCODED)
        .expect("should be ok");

    let got = unsafe { String::from_utf8_unchecked(got) }
        .escape_default()
        .to_string();

    let expect = "some data with \\u{0} and \\u{feff}";
    assert_eq!(expect, got);
}

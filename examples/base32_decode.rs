use encoding::base32;

fn main() {
    const ENCODED: &'static str = "ONXW2ZJAMRQXIYJAO5UXI2BAAAQGC3TEEDX3XPY=";
    const EXPECT: &'static str = "some data with \u{0} and \u{feff}";

    let got = base32::STD_ENCODING
        .decode_string(ENCODED)
        .map(|v| String::from_utf8_lossy(v.as_slice()).to_string())
        .unwrap();

    assert_eq!(EXPECT, got);
}

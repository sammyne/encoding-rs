use encoding::base32;

fn main() {
    let data = "any + old & data";
    const EXPECT: &'static str = "MFXHSIBLEBXWYZBAEYQGIYLUME======";
    let got = base32::STD_ENCODING.encode_to_string(data.as_bytes());

    assert_eq!(EXPECT, got);
}

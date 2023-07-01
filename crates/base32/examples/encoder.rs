use std::io::Write;

fn main() {
    const INPUT: &'static str = "foo\x00bar";
    const EXPECT: &'static str = "MZXW6ADCMFZA====";

    let mut out = vec![];
    let mut encoder = base32::new_encoder(*base32::STD_ENCODING, &mut out);

    let _ = encoder.write(INPUT.as_bytes());
    let _ = encoder.flush();
    std::mem::drop(encoder);

    let got = String::from_utf8_lossy(out.as_slice()).to_string();
    assert_eq!(EXPECT, got);
}

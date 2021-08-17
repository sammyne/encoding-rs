use std::io::Write;

use encoding::base32::{self, Encoder};

fn main() {
    const INPUT: &'static str = "foo\x00bar";
    const EXPECT: &'static str = "MZXW6ADCMFZA====";

    let mut out = vec![];
    let mut encoder = Encoder::new(base32::STD_ENCODING.clone(), &mut out);

    let _ = encoder.write(INPUT.as_bytes());
    let _ = encoder.flush();

    let got = String::from_utf8_lossy(out.as_slice()).to_string();
    assert_eq!(EXPECT, got);
}

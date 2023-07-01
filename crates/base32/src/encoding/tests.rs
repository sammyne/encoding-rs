use std::io::Write;

use crate::{Encoder, STD_ENCODING};

#[test]
fn without_padding_close() {
    let encodings = [
        *STD_ENCODING,
        STD_ENCODING.clone().with_padding(None).clone(),
    ];

    for (i, encoding) in encodings.iter().enumerate() {
        for (j, testpair) in testbot::PAIRS.iter().enumerate() {
            let mut buf = vec![];
            let mut encoder = Encoder::new(encoding.clone(), &mut buf);
            encoder.write(testpair.decoded).unwrap();
            encoder.flush().unwrap();

            let expected = if encoding.pad_char.is_some() {
                testpair.encoded.to_string()
            } else {
                testpair.encoded.replace('=', "")
            };

            let got = String::from_utf8_lossy(&buf).to_string();
            assert_eq!(
                expected, got,
                "#{}-#{} with padding({:?})",
                i, j, encoding.pad_char
            );
        }
    }
}

#[path = "../../tests/testbot/mod.rs"]
mod testbot;

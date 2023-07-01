use std::io::Read;

use crate::{Decoder, STD_ENCODING};

#[test]
fn decode_read_all() {
    let encodings = [
        *STD_ENCODING,
        STD_ENCODING.clone().with_padding(None).clone(),
    ];

    for (i, pair) in testbot::PAIRS.iter().enumerate() {
        for (j, encoding) in encodings.iter().enumerate() {
            let encoded = if encoding.pad_char.is_some() {
                pair.encoded.to_string()
            } else {
                pair.encoded.replace('=', "")
            };

            let mut dec_reader = vec![];
            Decoder::new(encoding.clone(), encoded.as_bytes())
                .read_to_end(&mut dec_reader)
                .unwrap();

            let expect = testbot::escape_ascii_string(pair.decoded);
            let got = testbot::escape_ascii_string(&dec_reader);

            assert_eq!(expect, got, "#{i}-#{j}");
        }
    }
}

#[test]
fn decode_small_buffer() {
    let encodings = [
        STD_ENCODING.clone(),
        STD_ENCODING.clone().with_padding(None).clone(),
    ];

    for bs in 1..200 {
        for (i, pair) in testbot::PAIRS.iter().enumerate() {
            for (j, encoding) in encodings.iter().enumerate() {
                let encoded = if encoding.pad_char.is_some() {
                    pair.encoded.to_string()
                } else {
                    pair.encoded.replace('=', "")
                };

                let mut decoder = Decoder::new(encoding.clone(), encoded.as_bytes());

                let mut all_read = vec![];
                loop {
                    let mut buf = vec![0u8; bs];
                    let n = decoder.read(&mut buf).unwrap();
                    all_read.extend_from_slice(&buf[..n]);
                    if n == 0 {
                        break;
                    }
                }

                let expect = testbot::escape_ascii_string(&pair.decoded);
                let got = testbot::escape_ascii_string(&all_read);

                assert_eq!(expect, got, "#{i}-#{j} decoded failed for buffer size={bs}");
            }
        }
    }
}

#[path = "../../tests/testbot/mod.rs"]
mod testbot;

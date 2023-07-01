use std::io::{Read, Write};

use ascii85::Decoder;

#[test]
fn big() {
    const N: usize = 3 * 1000 + 1;
    const ALPHA: &[u8] =
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

    let mut raw = [0u8; N];
    for i in 0..raw.len() {
        raw[i] = ALPHA[i % ALPHA.len()];
    }

    let mut encoded = vec![];
    let mut w = ascii85::new_encoder(&mut encoded);
    let nn = w.write(&raw).unwrap();
    assert_eq!(
        N,
        nn,
        "Encoder.write({})",
        testbot::escape_ascii_string(&raw)
    );
    std::mem::drop(w);

    let mut decoded = vec![];
    Decoder::new(encoded.as_slice())
        .read_to_end(&mut decoded)
        .unwrap();

    if raw != decoded.as_slice() {
        let mut i = 0usize;
        while i < decoded.len() && i < raw.len() {
            if decoded[i] != raw[i] {
                break;
            }
            i += 1;
        }
        panic!("Decode(Encode({}-byte string)) failed at offset {}", N, i);
    }
}

mod testbot;

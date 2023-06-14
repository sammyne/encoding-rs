use std::io::{Read, Write};

use base64::STD_ENCODING;

#[test]
fn big() {
    const N: usize = 3 * 1000 + 1;
    const ALPHA: &'static [u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut raw = [0u8; N];
    for i in 0..N {
        raw[i] = ALPHA[i % ALPHA.len()];
    }

    let mut encoded = vec![];
    let mut w = base64::new_encoder(*STD_ENCODING, &mut encoded);
    let nn = w.write(&raw).expect("write");
    assert_eq!(raw.len(), nn, "encoder.write(raw)");
    w.flush().expect("flush");
    std::mem::drop(w);

    let mut decoded = vec![];
    base64::new_decoder(*STD_ENCODING, encoded.as_slice())
        .read_to_end(&mut decoded)
        .map_err(|err| err.to_string())
        .expect("new_decoder(...).read_to_end()");

    let i = match raw.iter().zip(decoded.iter()).position(|(a, b)| a != b) {
        None if raw.len() != decoded.len() => Some(raw.len().min(decoded.len())),
        None => None,
        Some(i) => Some(i),
    };

    if let Some(i) = i {
        panic!(
            "decode(encode({}-byte string)) failed at offset {}",
            raw.len(),
            i
        );
    }
}

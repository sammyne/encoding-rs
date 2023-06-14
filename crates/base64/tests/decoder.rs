use std::io::Read;

use base64::{RAW_URL_ENCODING, STD_ENCODING, URL_ENCODING};

mod testbot;

#[test]
fn decoder() {
    for p in testbot::PAIRS.iter() {
        let mut decoder = base64::new_decoder(STD_ENCODING.clone(), p.encoded.as_bytes());
        let mut dbuf = vec![0u8; STD_ENCODING.decoded_len(p.encoded.len())];
        let count = decoder.read(dbuf.as_mut_slice()).expect("read failed");

        assert_eq!(p.decoded.len(), count, "read from '{}'", p.encoded);

        let got = &dbuf[..count];
        assert_eq!(p.decoded, got, "read from '{}'", p.escape_ascii_decoded);

        let n = decoder.read(dbuf.as_mut_slice()).expect("trigger EOF");
        assert_eq!(0, n, "read {} after EOF", p.encoded);
    }
}

#[test]
fn decoder_buffering() {
    let bigtest = &testbot::BIG_TEST;

    for bs in 1..=12 {
        let mut decoder = base64::new_decoder(STD_ENCODING.clone(), bigtest.encoded.as_bytes());
        let mut buf = vec![0u8; bigtest.decoded.len() + 12];

        let mut total = 0usize;
        while total < bigtest.decoded.len() {
            let n = decoder
                .read(&mut buf[total..total + bs])
                .expect(&format!("read from '{}' at pos {}", bigtest.encoded, total));
            total += n;
            if n == 0 {
                // eof
                break;
            }
        }

        let got = testbot::escape_ascii_string(&buf[..total]);
        assert_eq!(
            bigtest.escape_ascii_decoded, got,
            "decoding/{} of '{}'",
            bs, bigtest.encoded
        );
    }
}

#[test]
fn decoder_raw() {
    const SOURCE: &'static str = "AAAAAA";
    let want = [0u8; 4];

    // Direct
    let dec1 = RAW_URL_ENCODING
        .decode_string(SOURCE)
        .expect("RAW_URL_ENCODING.decode_string()");
    assert_eq!(want.as_ref(), dec1.as_slice(), "dec1 != want");

    // Through reader. Used to fail
    let mut dec2 = vec![];
    let _ = base64::new_decoder(*RAW_URL_ENCODING, SOURCE.as_bytes())
        .take(100)
        .read_to_end(&mut dec2)
        .expect(&format!("reading new_decoder(RAW_URL_ENCODING, {SOURCE})"));
    assert_eq!(want.as_ref(), dec2.as_slice(), "dec2 != want");

    let mut dec3 = vec![];
    let src = SOURCE.to_string() + "==";
    let _ = base64::new_decoder(*URL_ENCODING, src.as_bytes())
        .read_to_end(&mut dec3)
        .expect(&format!("reading new_decoder(URL_ENCODING, {src})"));
    assert_eq!(want.as_ref(), dec3.as_slice(), "dec3 != want");
}

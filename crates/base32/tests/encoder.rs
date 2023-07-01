use std::io::Write;

use base32::STD_ENCODING;

#[test]
fn buffering() {
    let bigtest = &testbot::BIGTEST;
    let input = bigtest.decoded;

    let decoded_escaped = testbot::escape_ascii_string(input);

    for bs in 1..=12 {
        let mut bb = vec![];
        let mut encoder = base32::new_encoder(*STD_ENCODING, &mut bb);
        for pos in (0..input.len()).step_by(bs) {
            let end = usize::min(pos + bs, input.len());
            let n = encoder.write(input[pos..end].as_ref()).unwrap();
            assert_eq!(
                end - pos,
                n,
                "buffer size {} gave wrong length at offset {}",
                bs,
                pos
            )
        }
        std::mem::drop(encoder);

        let got = String::from_utf8_lossy(bb.as_slice()).to_string();

        assert_eq!(
            bigtest.encoded, got,
            "Encoding/{} of {}",
            bs, decoded_escaped
        );
    }
}

#[test]
fn encoder() {
    for (i, p) in testbot::PAIRS.iter().enumerate() {
        let mut bb = vec![];
        let mut encoder = base32::new_encoder(*STD_ENCODING, &mut bb);
        let _ = encoder.write(p.decoded);
        let _ = encoder.flush(); // the flush is redundant because it's called once encoder is dropped.
        std::mem::drop(encoder);

        let got = String::from_utf8_lossy(bb.as_slice()).to_string();

        let decoded = testbot::escape_ascii_string(p.decoded);
        assert_eq!(p.encoded, got, "#{} encode({})", i, decoded);
    }
}

mod testbot;

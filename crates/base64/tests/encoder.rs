use std::io::Write;

use base64::STD_ENCODING;

mod testbot;

#[test]
fn encoder() {
    for (i, p) in testbot::PAIRS.iter().enumerate() {
        let mut bb = vec![];
        let mut encoder = base64::new_encoder(STD_ENCODING.clone(), &mut bb);
        let _ = encoder.write(p.decoded).unwrap();
        let _ = encoder.flush().unwrap();
        std::mem::drop(encoder);

        let got = unsafe { std::str::from_utf8_unchecked(bb.as_slice()) };
        assert_eq!(p.encoded, got, "#{} encode({})", i, p.escape_ascii_decoded);
    }
}

#[test]
fn encoder_buffering() {
    let bigtest = &testbot::BIG_TEST;

    let input = bigtest.decoded;
    for bs in 1..=12 {
        let mut bb = vec![];
        let mut encoder = base64::new_encoder(*STD_ENCODING, &mut bb);
        for pos in (0..input.len()).step_by(bs) {
            let end = (pos + bs).min(input.len());

            let chunk = unsafe { std::str::from_utf8_unchecked(&input[pos..end]) };
            let n = encoder
                .write(&input[pos..end])
                .expect(&format!("write {chunk}"));
            assert_eq!(end - pos, n, "write '{chunk}'");
        }
        encoder.flush().expect("flush");
        std::mem::drop(encoder);

        let got = testbot::escape_ascii_string(bb.as_slice());
        assert_eq!(
            bigtest.encoded, got,
            "encoding/{} of '{}'",
            bs, bigtest.escape_ascii_decoded
        );
    }
}

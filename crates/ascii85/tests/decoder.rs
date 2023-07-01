use std::io::Read;

#[test]
fn buffering() {
    let bigtest = *testbot::BIGTEST;
    for bs in 1..=12 {
        let mut decoder = ascii85::new_decoder(bigtest.encoded.as_bytes());
        let mut buf = vec![0u8; bigtest.decoded.len() + 12];

        let mut total = 0usize;
        while total < bigtest.decoded.len() {
            total += decoder.read(&mut buf[total..(total + bs)]).unwrap();
        }

        let got = String::from_utf8_lossy(&buf[0..total]);
        assert_eq!(
            bigtest.decoded, got,
            "decoding/{} of {}",
            bs, bigtest.encoded
        );
    }
}

#[test]
fn decoder() {
    for (i, p) in testbot::PAIRS.iter().enumerate().skip(1) {
        let encoded = p.encoded.as_bytes();
        let mut decoder = ascii85::new_decoder(encoded);
        let mut dbuf = vec![];
        decoder.read_to_end(&mut dbuf).unwrap();

        assert_eq!(
            p.decoded.len(),
            dbuf.len(),
            "#{} read from {}",
            i,
            p.encoded
        );

        let got = String::from_utf8_lossy(dbuf.as_slice()).to_string();
        assert_eq!(p.decoded, got, "#{} read from {}", i, p.encoded);
    }
}

#[test]
fn internal_whitespace() {
    let s = " ".repeat(2048) + "z";

    let mut decoded = vec![];
    ascii85::new_decoder(s.as_bytes())
        .read_to_end(&mut decoded)
        .unwrap();

    const EXPECT: [u8; 4] = [0u8; 4];
    assert_eq!(EXPECT, decoded.as_slice());
}

mod testbot;

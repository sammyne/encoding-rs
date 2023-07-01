#[test]
fn corrupt() {
    struct Corrupt<'a> {
        e: &'a str,
        p: usize,
    }
    let new_corrupt = |e: &'static str, p: usize| Corrupt { e, p };

    let examples = vec![new_corrupt("v", 0), new_corrupt("!z!!!!!!!!!", 1)];

    for (i, e) in examples.iter().enumerate() {
        let mut dbuf = vec![0u8; e.e.len() * 4];
        let got = ascii85::decode(&mut dbuf, e.e.as_bytes(), true).unwrap_err();

        assert_eq!(e.p, got.idx, "#{} corruption in {}", i, e.e);
    }
}

#[test]
fn decode() {
    for (i, p) in testbot::PAIRS.iter().enumerate() {
        let mut dbuf = vec![0u8; 4 * p.encoded.len()];
        let (ndst, nsrc) =
            ascii85::decode(dbuf.as_mut_slice(), p.encoded.as_bytes(), true).unwrap();
        assert_eq!(nsrc, p.encoded.len(), "#{i} bad encoded length");
        assert_eq!(ndst, p.decoded.len(), "#{i} bad decoded length");
        assert_eq!(
            String::from_utf8_lossy(&dbuf[..ndst]),
            p.decoded.to_string(),
            "#{i} invalid decoded string",
        );
    }
}

mod testbot;

#[test]
fn encode() {
    for (i, p) in testbot::PAIRS.iter().enumerate() {
        let mut buf = vec![0u8; ascii85::max_encoded_len(p.decoded.len())];
        let n = ascii85::encode(buf.as_mut_slice(), p.decoded.as_bytes());
        buf.resize(n, 0);
        assert_eq!(
            testbot::strip85(buf.as_slice()),
            testbot::strip85(p.encoded.as_bytes()),
            "#{i}"
        );
    }
}

mod testbot;

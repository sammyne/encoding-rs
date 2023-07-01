use ascii85::Encoder;

use std::io::Write;

#[test]
fn buffering() {
    let bigtest = *testbot::BIGTEST;

    let input = bigtest.decoded.as_bytes();
    for bs in 1..=12 {
        let mut buf: Vec<u8> = vec![];
        let mut encoder = Encoder::new(&mut buf);
        for pos in (0..input.len()).step_by(bs) {
            let end = usize::min(pos + bs, input.len());
            let n = encoder.write(&input[pos..end]).unwrap();
            assert_eq!(
                end - pos,
                n,
                "write({}) got invalid length",
                testbot::strip85(&input[pos..end]),
            );
        }

        encoder.flush().unwrap();

        let expect = testbot::strip85(bigtest.encoded.as_bytes());
        let got = testbot::strip85(buf.as_slice());
        assert_eq!(expect, got);
    }
}

#[test]
fn encoder() {
    for (i, p) in testbot::PAIRS.iter().enumerate() {
        let mut buf: Vec<u8> = vec![];
        let mut encoder = Encoder::new(&mut buf);
        let _ = encoder.write(p.decoded.as_bytes());
        let _ = encoder.flush();

        let expect = testbot::strip85(p.encoded.as_bytes());
        let got = testbot::strip85(buf.as_slice());
        assert_eq!(expect, got, "#{i}");
    }
}

mod testbot;

use crate::ascii85::{self, Decoder, Encoder};

use crate::Error;

use std::io::{Read, Write};
use std::panic;

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
    let mut w = Encoder::new(&mut encoded);
    let nn = w.write(&raw).unwrap();
    assert_eq!(N, nn, "Encoder.write(raw)");
    w.flush().unwrap();

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

#[test]
fn decode() {
    for (i, p) in pairs().iter().enumerate() {
        let mut dbuf = vec![0u8; 4 * p.encoded.len()];
        let (ndst, nsrc) =
            ascii85::decode(dbuf.as_mut_slice(), p.encoded.as_bytes(), true).unwrap();
        assert_eq!(nsrc, p.encoded.len(), "#{} bad encoded length", i);
        assert_eq!(ndst, p.decoded.len(), "#{} bad decoded length", i);
        assert_eq!(
            String::from_utf8_lossy(&dbuf[..ndst]),
            p.decoded.to_string(),
            "#{} invalid decoded string",
            i,
        );
    }
}

#[test]
fn decode_corrupt() {
    struct Corrupt<'a> {
        e: &'a str,
        p: usize,
    }
    let new_corrupt = |e: &'static str, p: usize| Corrupt { e, p };

    let examples = vec![new_corrupt("v", 0), new_corrupt("!z!!!!!!!!!", 1)];

    for (i, e) in examples.iter().enumerate() {
        let mut dbuf = vec![0u8; e.e.len() * 4];
        match ascii85::decode(&mut dbuf, e.e.as_bytes(), true).expect_err("") {
            Error::CorruptInputError(_, offset) => {
                assert_eq!(offset, e.p, "#{} corruption in {}", i, e.e)
            }
            _ => panic!(
                "#{} decoder failed to detect corruption in ({},{})",
                i, e.e, e.p
            ),
        }
    }
}

#[test]
fn decoder() {
    for (i, p) in pairs().iter().enumerate().skip(1) {
        let encoded = p.encoded.as_bytes();
        let mut decoder = Decoder::new(encoded);
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
fn decoder_buffering() {
    let bigtest = bigtest();
    for bs in 1..=12 {
        let mut decoder = Decoder::new(bigtest.encoded.as_bytes());
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
fn decoder_internal_whitespace() {
    let s = " ".repeat(2048) + "z";

    let mut decoded = vec![];
    Decoder::new(s.as_bytes())
        .read_to_end(&mut decoded)
        .unwrap();

    const EXPECT: [u8; 4] = [0u8; 4];
    assert_eq!(EXPECT, decoded.as_slice());
}

#[test]
fn encode() {
    for (i, p) in pairs().iter().enumerate() {
        let mut buf = vec![0u8; ascii85::max_encoded_len(p.decoded.len())];
        let n = ascii85::encode(buf.as_mut_slice(), p.decoded.as_bytes());
        buf.resize(n, 0);
        assert_eq!(
            strip85(buf.as_slice()),
            strip85(p.encoded.as_bytes()),
            "#{}",
            i
        );
    }
}

#[test]
fn encoder() {
    for (i, p) in pairs().iter().enumerate() {
        let mut buf: Vec<u8> = vec![];
        let mut encoder = Encoder::new(&mut buf);
        let _ = encoder.write(p.decoded.as_bytes());
        let _ = encoder.flush();

        let expect = strip85(p.encoded.as_bytes());
        let got = strip85(buf.as_slice());
        assert_eq!(expect, got, "#{}", i);
    }
}

#[test]
fn encoder_buffering() {
    let bigtest = bigtest();

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
                strip85(&input[pos..end]),
            );
        }

        encoder.flush().unwrap();

        let expect = strip85(bigtest.encoded.as_bytes());
        let got = strip85(buf.as_slice());
        assert_eq!(expect, got);
    }
}

struct Testpair<'a> {
    decoded: &'a str,
    encoded: &'a str,
}

impl<'a> Testpair<'a> {
    fn new(decoded: &'a str, encoded: &'a str) -> Self {
        Self { decoded, encoded }
    }
}

fn bigtest() -> Testpair<'static> {
    Testpair::new(
        concat!(
            "Man is distinguished, not only by his reason, but by this singular passion from ",
            "other animals, which is a lust of the mind, that by a perseverance of delight in ",
            "the continued and indefatigable generation of knowledge, exceeds the short ",
            "vehemence of any carnal pleasure."
        ),
        concat!(
            "9jqo^BlbD-BleB1DJ+*+F(f,q/0JhKF<GL>Cj@.4Gp$d7F!,L7@<6@)/0JDEF<G%<+EV:2F!,\n",
            "O<DJ+*.@<*K0@<6L(Df-\\0Ec5e;DffZ(EZee.Bl.9pF\"AGXBPCsi+DGm>@3BB/F*&OCAfu2/AKY\n",
            "i(DIb:@FD,*)+C]U=@3BN#EcYf8ATD3s@q?d$AftVqCh[NqF<G:8+EV:.+Cf>-FD5W8ARlolDIa\n",
            "l(DId<j@<?3r@:F%a+D58'ATD4$Bl@l3De:,-DJs`8ARoFb/0JMK@qB4^F!,R<AKZ&-DfTqBG%G\n",
            ">uD.RTpAKYo'+CT/5+Cei#DII?(E,9)oF*2M7/c\n"
        ),
    )
}

fn pairs() -> Vec<Testpair<'static>> {
    vec![
        // Encode returns 0 when len(src) is 0
        Testpair::new("", ""),
        // Wikipedia example
        bigtest(),
        // Special case when shortening !!!!! to z.
        Testpair::new("\0\0\0\0", "z"),
    ]
}

fn strip85(s: &[u8]) -> String {
    let mut out = String::with_capacity(s.len());
    for &v in s {
        if v > b' ' {
            out.push(v as char);
        }
    }

    out
}

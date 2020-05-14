use crate::base64::{self, Encoding};

#[test]
fn encode() {
    let pairs = pairs();
    let test_vector = encoding_tests();
    for (i, p) in pairs.iter().enumerate() {
        // non-strict ones
        for tt in &test_vector {
            let got = tt.enc.encode_to_string(p.decoded.as_bytes());
            let expect = (tt.conv)(p.encoded);
            assert_eq!(
                expect, got,
                "#{} encode({}) failed: expect {}, got {}",
                i, p.decoded, expect, got
            );
        }

        // strict ones
        for tt in &test_vector {
            let enc = {
                let mut enc = tt.enc.clone();
                enc.strict();
                enc
            };

            let got = enc.encode_to_string(p.decoded.as_bytes());
            let expect = (tt.conv)(p.encoded);
            assert_eq!(
                expect, got,
                "#{} encode({}) failed: expect {}, got {}",
                i, p.decoded, expect, got
            );
        }
    }
}

#[test]
fn decode() {
    let pairs = pairs();
    let test_vector = encoding_tests();
    for (i, p) in pairs.iter().enumerate() {
        for tt in &test_vector[..1] {
            let encoded = (tt.conv)(p.encoded);

            {
                let mut dbuf = vec![0u8; tt.enc.decoded_len(encoded.len())];
                let n = tt
                    .enc
                    .decode(dbuf.as_mut_slice(), encoded.as_bytes())
                    .expect("unexpected error");
                assert_eq!(
                    p.decoded.len(),
                    n,
                    "#{} decode({}): expect length {}, got {}",
                    i,
                    encoded,
                    p.decoded.len(),
                    n
                );

                let got = unsafe { std::str::from_utf8_unchecked(&dbuf[..n]) };
                assert_eq!(
                    p.decoded, got,
                    "#{} decode({}): expect output {}, got {}",
                    i, encoded, p.decoded, got
                );
            }

            {
                let got = tt.enc.decode_string(&encoded).expect("unexpected error");
                let got = unsafe { std::str::from_utf8_unchecked(&got[..]) };
                assert_eq!(
                    p.decoded, got,
                    "#{} decode_string({}): expect output {}, got {}",
                    i, encoded, p.decoded, got
                );
            }
        }
    }
}

struct EncodingTest {
    enc: Encoding,
    conv: fn(&str) -> String,
}

struct Testpair<'a> {
    decoded: &'a str,
    encoded: &'a str,
}

impl EncodingTest {
    fn new(enc: Encoding, conv: fn(&str) -> String) -> Self {
        Self { enc, conv }
    }
}

impl<'a> Testpair<'a> {
    fn new(decoded: &'a str, encoded: &'a str) -> Self {
        Self { decoded, encoded }
    }
}

fn encoding_tests() -> Vec<EncodingTest> {
    vec![
        EncodingTest::new(base64::STD_ENCODING.clone(), std_ref),
        EncodingTest::new(base64::URL_ENCODING.clone(), url_ref),
        EncodingTest::new(base64::RAW_STD_ENCODING.clone(), raw_ref),
        EncodingTest::new(base64::RAW_URL_ENCODING.clone(), raw_url_ref),
        EncodingTest::new(new_funny_encoding(), funny_ref),
    ]
}

fn funny_ref(r: &str) -> String {
    r.to_string().replace("=", "@")
}

fn new_funny_encoding() -> Encoding {
    let mut enc = base64::STD_ENCODING.clone();

    enc.with_padding('@');

    enc
}

fn pairs() -> Vec<Testpair<'static>> {
    vec![
        // RFC 3548 examples
        Testpair::new(
            unsafe { std::str::from_utf8_unchecked(b"\x14\xfb\x9c\x03\xd9\x7e") },
            "FPucA9l+",
        ),
        Testpair::new(
            unsafe { std::str::from_utf8_unchecked(b"\x14\xfb\x9c\x03\xd9") },
            "FPucA9k=",
        ),
        Testpair::new(
            unsafe { std::str::from_utf8_unchecked(b"\x14\xfb\x9c\x03") },
            "FPucAw==",
        ),
        // RFC 4648 examples
        Testpair::new("", ""),
        Testpair::new("f", "Zg=="),
        Testpair::new("fo", "Zm8="),
        Testpair::new("foo", "Zm9v"),
        Testpair::new("foob", "Zm9vYg=="),
        Testpair::new("fooba", "Zm9vYmE="),
        Testpair::new("foobar", "Zm9vYmFy"),
        // Wikipedia examples
        Testpair::new("sure.", "c3VyZS4="),
        Testpair::new("sure", "c3VyZQ=="),
        Testpair::new("sur", "c3Vy"),
        Testpair::new("su", "c3U="),
        Testpair::new("leasure.", "bGVhc3VyZS4="),
        Testpair::new("easure.", "ZWFzdXJlLg=="),
        Testpair::new("asure.", "YXN1cmUu"),
        Testpair::new("sure.", "c3VyZS4="),
    ]
}

fn raw_ref(r: &str) -> String {
    r.trim_end_matches('=').to_string()
}

fn raw_url_ref(r: &str) -> String {
    let rr = url_ref(r);
    raw_ref(&rr)
}

fn std_ref(r: &str) -> String {
    r.to_string()
}

fn url_ref(r: &str) -> String {
    r.to_string().replace("+", "-").replace("/", "_")
}

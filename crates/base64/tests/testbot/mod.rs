use lazy_static::lazy_static;

use base64::Encoding;

lazy_static! {
    pub static ref BIG_TEST: Testpair = Testpair::new(
        b"Twas brillig, and the slithy toves",
        "VHdhcyBicmlsbGlnLCBhbmQgdGhlIHNsaXRoeSB0b3Zlcw==");

    pub static ref ENCODING_TESTS: Vec<EncodingTest> = vec![
        EncodingTest::new(*base64::STD_ENCODING, std_ref),
        EncodingTest::new(*base64::URL_ENCODING, url_ref),
        EncodingTest::new(*base64::RAW_STD_ENCODING, raw_ref),
        EncodingTest::new(*base64::RAW_URL_ENCODING, raw_url_ref),
        EncodingTest::new(new_funny_encoding(), funny_ref),
    ];

  pub static ref PAIRS: Vec<Testpair> = vec![
        // RFC 3548 examples
        Testpair::new(
            b"\x14\xfb\x9c\x03\xd9\x7e",
            "FPucA9l+",
        ),
        Testpair::new(
            b"\x14\xfb\x9c\x03\xd9",
            "FPucA9k=",
        ),
        Testpair::new(
            b"\x14\xfb\x9c\x03",
            "FPucAw==",
        ),
        // RFC 4648 examples
        Testpair::new(b"", ""),
        Testpair::new(b"f", "Zg=="),
        Testpair::new(b"fo", "Zm8="),
        Testpair::new(b"foo", "Zm9v"),
        Testpair::new(b"foob", "Zm9vYg=="),
        Testpair::new(b"fooba", "Zm9vYmE="),
        Testpair::new(b"foobar", "Zm9vYmFy"),
        // Wikipedia examples
        Testpair::new(b"sure.", "c3VyZS4="),
        Testpair::new(b"sure", "c3VyZQ=="),
        Testpair::new(b"sur", "c3Vy"),
        Testpair::new(b"su", "c3U="),
        Testpair::new(b"leasure.", "bGVhc3VyZS4="),
        Testpair::new(b"easure.", "ZWFzdXJlLg=="),
        Testpair::new(b"asure.", "YXN1cmUu"),
        Testpair::new(b"sure.", "c3VyZS4="),
    ];
}

pub struct EncodingTest {
    pub enc: Encoding,
    pub conv: fn(&str) -> String,
}

pub struct Testpair {
    pub decoded: &'static [u8],
    pub encoded: &'static str,

    pub escape_ascii_decoded: String,
}

impl EncodingTest {
    fn new(enc: Encoding, conv: fn(&str) -> String) -> Self {
        Self { enc, conv }
    }
}

impl Testpair {
    fn new(decoded: &'static [u8], encoded: &'static str) -> Self {
        let escape_ascii_decoded: String = escape_ascii_string(decoded);
        Self {
            decoded,
            encoded,
            escape_ascii_decoded,
        }
    }
}

pub fn escape_ascii_string(s: &[u8]) -> String {
    s.iter().map(|v| v.escape_ascii().to_string()).collect()
}

fn funny_ref(r: &str) -> String {
    r.to_string().replace("=", "@")
}

fn new_funny_encoding() -> Encoding {
    let mut enc = base64::STD_ENCODING.clone();

    enc.with_padding('@');

    enc
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

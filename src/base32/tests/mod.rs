pub use crate::base32::STD_ENCODING;

#[test]
fn decode() {
    let pairs = pairs();
    for (i, v) in pairs.iter().enumerate() {
        let mut got = "-".repeat(STD_ENCODING.decoded_len(v.encoded.len()));
        let n = STD_ENCODING
            .decode(unsafe { got.as_bytes_mut() }, v.encoded.as_bytes())
            .unwrap();
        got.truncate(n);
        assert_eq!(v.decoded, got, "#{}", i);
    }
}

#[test]
fn decode_string() {
    for (i, v) in pairs().iter().enumerate() {
        let got = STD_ENCODING.decode_string(v.encoded).unwrap();
        assert_eq!(v.decoded.as_bytes(), got.as_slice(), "#{}", i);
    }
}

#[test]
fn encode() {
    let pairs = pairs();
    for (i, v) in pairs.iter().enumerate() {
        let got = STD_ENCODING.encode_to_string(v.decoded.as_bytes());
        assert_eq!(v.encoded, got, "#{}", i);
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

fn pairs() -> Vec<Testpair<'static>> {
    vec![
        // RFC 4648 examples
        Testpair::new("", ""),
        Testpair::new("f", "MY======"),
        Testpair::new("fo", "MZXQ===="),
        Testpair::new("foo", "MZXW6==="),
        Testpair::new("foob", "MZXW6YQ="),
        Testpair::new("fooba", "MZXW6YTB"),
        Testpair::new("foobar", "MZXW6YTBOI======"),
        // Wikipedia examples, converted to base32
        Testpair::new("sure.", "ON2XEZJO"),
        Testpair::new("sure", "ON2XEZI="),
        Testpair::new("sur", "ON2XE==="),
        Testpair::new("su", "ON2Q===="),
        Testpair::new("leasure.", "NRSWC43VOJSS4==="),
        Testpair::new("easure.", "MVQXG5LSMUXA===="),
        Testpair::new("asure.", "MFZXK4TFFY======"),
        Testpair::new("sure.", "ON2XEZJO"),
    ]
}

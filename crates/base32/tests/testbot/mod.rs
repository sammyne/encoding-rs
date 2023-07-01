lazy_static::lazy_static! {
    pub static ref BIGTEST: Testpair = Testpair::new(
        b"Twas brillig, and the slithy toves",
        "KR3WC4ZAMJZGS3DMNFTSYIDBNZSCA5DIMUQHG3DJORUHSIDUN53GK4Y=",
    );

  pub static ref PAIRS: Vec<Testpair> = vec![
      // RFC 4648 examples
      Testpair::new(b"", ""),
      Testpair::new(b"f", "MY======"),
      Testpair::new(b"fo", "MZXQ===="),
      Testpair::new(b"foo", "MZXW6==="),
      Testpair::new(b"foob", "MZXW6YQ="),
      Testpair::new(b"fooba", "MZXW6YTB"),
      Testpair::new(b"foobar", "MZXW6YTBOI======"),
      // Wikipedia ebxamples, converted to base32
      Testpair::new(b"sure.", "ON2XEZJO"),
      Testpair::new(b"sure", "ON2XEZI="),
      Testpair::new(b"sur", "ON2XE==="),
      Testpair::new(b"su", "ON2Q===="),
      Testpair::new(b"leasure.", "NRSWC43VOJSS4==="),
      Testpair::new(b"easure.", "MVQXG5LSMUXA===="),
      Testpair::new(b"asure.", "MFZXK4TFFY======"),
      Testpair::new(b"sure.", "ON2XEZJO"),
  ];
}

pub struct Testpair {
    pub decoded: &'static [u8],
    pub encoded: &'static str,
}

impl Testpair {
    fn new(decoded: &'static [u8], encoded: &'static str) -> Self {
        Self { decoded, encoded }
    }
}

#[allow(dead_code)]
pub fn escape_ascii_string(s: &[u8]) -> String {
    s.iter().map(|v| v.escape_ascii().to_string()).collect()
}

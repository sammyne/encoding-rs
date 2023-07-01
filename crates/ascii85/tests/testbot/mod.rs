lazy_static::lazy_static! {
  pub static ref BIGTEST: Testpair = Testpair::new(
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
    );

  pub static ref PAIRS: Vec<Testpair> = vec![
     // Encode returns 0 when len(src) is 0
        Testpair::new("", ""),
        // Wikipedia example
        *BIGTEST,
        // Special case when shortening !!!!! to z.
        Testpair::new("\0\0\0\0", "z"),
  ];
}

#[derive(Clone, Copy)]
pub struct Testpair {
    pub decoded: &'static str,
    pub encoded: &'static str,
}

impl Testpair {
    fn new(decoded: &'static str, encoded: &'static str) -> Self {
        Self { decoded, encoded }
    }
}

#[allow(dead_code)]
pub fn escape_ascii_string(s: &[u8]) -> String {
    s.iter().map(|v| v.escape_ascii().to_string()).collect()
}

#[allow(dead_code)]
pub fn strip85(s: &[u8]) -> String {
    let mut out = String::with_capacity(s.len());
    for &v in s {
        if v > b' ' {
            out.push(v as char);
        }
    }

    out
}

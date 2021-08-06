use crate::ascii85;

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
fn encode() {
    for (i, p) in pairs().iter().enumerate() {
        println!(
            "{}-{}",
            p.decoded.len(),
            ascii85::max_encoded_len(p.decoded.len())
        );
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

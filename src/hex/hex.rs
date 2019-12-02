use super::errors::Error;

const HEXTABLE: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

pub fn decode(dst: &mut [u8], src: &[u8]) -> Result<usize, Error> {
    let mut i: usize = 0;

    for j in (1..src.len()).filter(|idx| (idx % 2 == 1)) {
        let a = from_hex_char(src[j - 1]).map_err(|_| Error::InvalidByteError(i, src[j - 1]))?;
        let b = from_hex_char(src[j]).map_err(|_| Error::InvalidByteError(i, src[j]))?;

        dst[i] = (a << 4) | b;
        i += 1;
    }

    if src.len() % 2 == 1 {
        let j = src.len() - 1;
        from_hex_char(src[j]).map_err(|_| Error::InvalidByteError(i, src[j]))?;
    }

    Ok(i)
}

pub fn decode_string(s: &str) -> Result<Vec<u8>, Error> {
    let mut dst = vec![0; s.len() / 2];

    let ell = decode(dst.as_mut_slice(), s.as_bytes())?;

    dst.resize(ell, 0);

    Ok(dst)
}

pub fn encode(dst: &mut [u8], src: &[u8]) -> usize {
    let mut j = 0;
    for v in src.iter() {
        let vv = *v as usize;

        dst[j] = HEXTABLE[vv >> 4];
        dst[j + 1] = HEXTABLE[vv & 0x0f];

        j += 2;
    }

    src.len() * 2
}

pub fn encode_len(n: usize) -> usize {
    n * 2
}

pub fn encode_to_string(src: &[u8]) -> String {
    let mut dst = vec![0; encode_len(src.len())];

    encode(dst.as_mut_slice(), src);

    String::from_utf8(dst).unwrap()
}

fn from_hex_char(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn from_hex_char_ok() {
        for v in b'0'..b'9' {
            let got = super::from_hex_char(v).unwrap();
            let expect = v - b'0';

            assert_eq!(expect, got);
        }

        for v in b'a'..b'f' {
            let got = super::from_hex_char(v).unwrap();
            let expect = v - b'a' + 10;

            assert_eq!(expect, got);
        }

        for v in b'A'..b'F' {
            let got = super::from_hex_char(v).unwrap();
            let expect = v - b'A' + 10;

            assert_eq!(expect, got);
        }
    }
}

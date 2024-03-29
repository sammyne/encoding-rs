use super::errors::Error;

const HEXTABLE: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f',
];

/// Decodes src into [decoded_len(src.len())][decoded_len] bytes,
/// returning the actual number of bytes written to `dst`.
///
/// `decode` expects that `src` contains only hexadecimal
/// characters and that `src` has even length.
/// If the input is malformed, `decode` returns the number
/// of bytes decoded before the error.
///
/// # Example
/// ```
#[doc = include_str!("../examples/decode.rs")]
/// ```
pub fn decode(dst: &mut [u8], src: &[u8]) -> Result<usize, (Error, usize)> {
    let mut i: usize = 0;

    for j in (1..src.len()).step_by(2) {
        let (p, q) = (src[j - 1], src[j]);

        let a = from_hex_char(p).map_err(|_| (Error::InvalidByte(p), i))?;
        let b = from_hex_char(q).map_err(|_| (Error::InvalidByte(q), i))?;

        dst[i] = (a << 4) | b;
        i += 1;
    }

    if src.len() % 2 == 1 {
        let j = src.len() - 1;
        from_hex_char(src[j]).map_err(|_| (Error::InvalidByte(src[j]), j))?;
        return Err((Error::Length, i));
    }

    Ok(i)
}

/// Returns the length of a decoding of `x` source bytes.
/// Specifically, it returns `x / 2`.
pub fn decoded_len(x: usize) -> usize {
    x / 2
}

/// Returns the bytes represented by the hexadecimal string `s`.
///
/// `decode_string` expects that src contains only hexadecimal
/// characters and that src has even length.
/// If the input is malformed, `decode_string` returns
/// the bytes decoded before the error.
///
/// # Example
/// ```
#[doc = include_str!("../examples/decode_string.rs")]
/// ```
pub fn decode_string(s: &str) -> Result<Vec<u8>, (Error, Vec<u8>)> {
    let mut dst = vec![0; s.len() / 2];

    match decode(dst.as_mut_slice(), s.as_bytes()) {
        Ok(_) => Ok(dst),
        Err((err, ok_len)) => {
            dst.resize(ok_len, 0);
            Err((err, dst))
        }
    }
}

/// Encodes `src` into [encoded_len(src.len())][encoded_len]
/// bytes of `dst`. As a convenience, it returns the number
/// of bytes written to `dst`, but this value is always
/// [encoded_len(src.len())][encoded_len].
/// `encode` implements hexadecimal encoding.
///
/// # Example
/// ```
#[doc = include_str!("../examples/encode.rs")]
/// ```
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

/// Returns the length of an encoding of `n` source bytes.
/// Specifically, it returns `n * 2`.
pub fn encoded_len(n: usize) -> usize {
    n * 2
}

/// Returns the hexadecimal encoding of `src`.
///
/// # Example
/// ```
#[doc = include_str!("../examples/encode_to_string.rs")]
/// ```
pub fn encode_to_string(src: &[u8]) -> String {
    let mut dst = vec![0; encoded_len(src.len())];

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

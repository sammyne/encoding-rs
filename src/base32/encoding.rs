use lazy_static::lazy_static;

use super::constants;
use crate::Error;

lazy_static! {
    pub static ref STD_ENCODING: Encoding = Encoding::new(constants::ENCODE_STD);
    pub static ref HEX_ENCODING: Encoding = Encoding::new(constants::ENCODE_HEX);
}

#[derive(Clone)]
pub struct Encoding {
    encode: [u8; 32],
    decode_map: [u8; 256],
    pad_char: Option<u8>,
}

impl Encoding {
    pub fn new<T>(encoder: T) -> Self
    where
        T: AsRef<str>,
    {
        let encoder = encoder.as_ref().as_bytes();
        // @TODO: check duplicates
        if encoder.len() != 32 {
            panic!("encoding alphabet is not 32-bytes long")
        }

        let encode = {
            let mut e = [0u8; 32];
            e.copy_from_slice(encoder);
            e
        };

        let mut decode_map = [0u8; 256];
        for v in &mut decode_map {
            *v = 0xFF;
        }
        for (i, v) in encoder.iter().enumerate() {
            decode_map[(*v) as usize] = i as u8;
        }

        Self {
            encode,
            decode_map,
            pad_char: Some(constants::STD_PADDING),
        }
    }

    pub fn decode(&self, dst: &mut [u8], src: &[u8]) -> Result<usize, Error> {
        if src.len() == 0 {
            return Ok(0);
        }

        let mut buf = vec![0u8; src.len()];
        let l = strip_newlines(&mut buf, src);
        self.decode_(dst, &buf[..l]).map(|(n, _)| n)
    }

    pub fn decoded_len(&self, n: usize) -> usize {
        if self.pad_char.is_none() {
            n * 5 / 8
        } else {
            n / 8 * 5
        }
    }

    pub fn decode_string(&self, s: &str) -> Result<Vec<u8>, Error> {
        let mut out = vec![0u8; self.decoded_len(s.len())];
        let n = self.decode(out.as_mut_slice(), s.as_bytes())?;
        out.resize(n, 0);

        Ok(out)
    }

    pub fn encode(&self, dst: &mut [u8], src: &[u8]) {
        let (mut dst, mut src) = (dst, src);
        while src.len() > 0 {
            let mut b = [0u8; 8];

            // Unpack 8x 5-bit source blocks into a 5 byte destination quantum
            if src.len() >= 5 {
                b[7] = src[4] & 0x1F;
                b[6] = src[4] >> 5;
            }
            if src.len() >= 4 {
                b[6] |= (src[3] << 3) & 0x1F;
                b[5] = (src[3] >> 2) & 0x1F;
                b[4] = src[3] >> 7;
            }
            if src.len() >= 3 {
                b[4] |= (src[2] << 1) & 0x1F;
                b[3] = (src[2] >> 4) & 0x1F;
            }
            if src.len() >= 2 {
                b[3] |= (src[1] << 4) & 0x1F;
                b[2] = (src[1] >> 1) & 0x1F;
                b[1] = (src[1] >> 6) & 0x1F;
            }
            if src.len() >= 1 {
                b[1] |= (src[0] << 2) & 0x1F;
                b[0] = src[0] >> 3;
            }

            // Encode 5-bit blocks using the base32 alphabet
            let size = dst.len();
            if size >= 8 {
                // Common case, unrolled for extra performance
                dst[0] = self.encode[(b[0] & 0x1F) as usize];
                dst[1] = self.encode[(b[1] & 0x1F) as usize];
                dst[2] = self.encode[(b[2] & 0x1F) as usize];
                dst[3] = self.encode[(b[3] & 0x1F) as usize];
                dst[4] = self.encode[(b[4] & 0x1F) as usize];
                dst[5] = self.encode[(b[5] & 0x1F) as usize];
                dst[6] = self.encode[(b[6] & 0x1F) as usize];
                dst[7] = self.encode[(b[7] & 0x1F) as usize];
            } else {
                for i in 0..size {
                    dst[i] = self.encode[(b[i] & 0x1F) as usize];
                }
            }

            // Pad the final quantum
            if src.len() < 5 {
                if self.pad_char.is_none() {
                    break;
                }

                let padding = self.pad_char.unwrap();
                dst[7] = padding;
                if src.len() < 4 {
                    dst[6] = padding;
                    dst[5] = padding;
                    if src.len() < 3 {
                        dst[4] = padding;
                        if src.len() < 2 {
                            dst[3] = padding;
                            dst[2] = padding;
                        }
                    }
                }

                break;
            }

            src = &src[5..];
            dst = &mut dst[8..];
        }
    }

    pub fn encode_to_string(&self, src: &[u8]) -> String {
        let mut out = " ".repeat(self.encoded_len(src.len()));
        self.encode(unsafe { out.as_bytes_mut() }, src);
        out
    }

    pub fn encoded_len(&self, n: usize) -> usize {
        if self.pad_char.is_none() {
            (n * 8 + 4) / 5
        } else {
            (n + 4) / 5 * 8
        }
    }

    pub fn with_padding(&mut self, padding: Option<u8>) -> &mut Self {
        if padding.is_none() {
            self.pad_char = None;
            return self;
        }

        let c = padding.unwrap();
        if c == constants::CR || c == constants::LF {
            panic!("invalid padding")
        }

        if self.encode.iter().any(|&v| v == c) {
            panic!("padding contained in alphabet");
        }

        self.pad_char = Some(c);

        self
    }

    fn decode_(&self, dst: &mut [u8], src: &[u8]) -> Result<(usize, bool), Error> {
        let mut dsti = 0usize;
        let (mut src, olen) = (src, src.len());

        let (mut written, mut end) = (0usize, false);
        let pad_char = self.pad_char.unwrap_or(0);
        while src.len() > 0 && !end {
            // Decode quantum using the base32 alphabet
            let mut dbuf = [0u8; 8];
            let mut dlen = 8usize;

            for j in 0..8 {
                if src.len() == 0 {
                    if self.pad_char.is_some() {
                        // We have reached the end and are missing padding
                        return Err(new_corrupted_error(olen - src.len() - j));
                    }
                    // We have reached the end and are not expecting any padding
                    dlen = j;
                    end = true;
                    break;
                }

                let c = src[0];
                src = &src[1..];
                if (c == pad_char) && (j >= 2) && (src.len() < 8) {
                    // We've reached the end and there's padding
                    if src.len() + j < 8 - 1 {
                        // not enough padding
                        return Err(new_corrupted_error(olen));
                    }

                    for k in 0..(8 - 1 - j) {
                        if (src.len() < k) && (src[k] != pad_char) {
                            // incorrect padding
                            return Err(new_corrupted_error(olen - src.len() + k - 1));
                        }
                    }
                    dlen = j;
                    end = true;

                    // 7, 5 and 2 are not valid padding lengths, and so 1, 3 and 6 are not
                    // valid dlen values. See RFC 4648 Section 6 "Base 32 Encoding" listing
                    // the five valid padding lengths, and Section 9 "Illustrations and
                    // Examples" for an illustration for how the 1st, 3rd and 6th base32
                    // src bytes do not yield enough information to decode a dst byte.
                    match dlen {
                        1 | 3 | 6 => return Err(new_corrupted_error(olen - src.len() - 1)),
                        _ => {}
                    }
                    break;
                }
                dbuf[j] = self.decode_map[c as usize];
                if dbuf[j] == 0xFF {
                    return Err(new_corrupted_error(olen - src.len() - 1));
                }
            }

            // Pack 8x 5-bit source blocks into 5 byte destination quantum
            if dlen >= 8 {
                dst[dsti + 4] = (dbuf[6] << 5) | dbuf[7];
                written += 1;
            }
            if dlen >= 7 {
                dst[dsti + 3] = (dbuf[4] << 7) | (dbuf[5] << 2) | (dbuf[6]) >> 3;
                written += 1;
            }
            if dlen >= 5 {
                dst[dsti + 2] = (dbuf[3] << 4) | (dbuf[4] >> 1);
                written += 1;
            }
            if dlen >= 4 {
                dst[dsti + 1] = (dbuf[1] << 6) | (dbuf[2] << 1) | (dbuf[3] >> 4);
                written += 1;
            }
            if dlen >= 2 {
                dst[dsti + 0] = (dbuf[0] << 3) | dbuf[1] >> 2;
                written += 1;
            }
            dsti += 5;
        }

        Ok((written, end))
    }
}

fn new_corrupted_error(idx: usize) -> Error {
    Error::CorruptInputError("base32", idx)
}

/*
fn new_io_error<E>(err: E, n: usize) -> Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    Error::IO(io::Error::new(io::ErrorKind::Other, err), n)
}
*/

fn strip_newlines(dst: &mut [u8], src: &[u8]) -> usize {
    let mut offset = 0usize;
    for &c in src {
        if c == constants::CR || c == constants::LF {
            continue;
        }
        dst[offset] = c;
        offset += 1;
    }

    offset
}

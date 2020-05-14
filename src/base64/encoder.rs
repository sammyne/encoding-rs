use std::io;

use crate::binary::{BigEndian, ByteOrder};
use crate::Error;

use super::constants;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref RAW_STD_ENCODING: Encoding = {
        let mut v = Encoding::new(constants::ENCODE_STD);
        v.without_padding();
        v
    };
    pub static ref RAW_URL_ENCODING: Encoding = {
        let mut v = Encoding::new(constants::ENCODE_URL);
        v.without_padding();
        v
    };
    pub static ref STD_ENCODING: Encoding = Encoding::new(constants::ENCODE_STD);
    pub static ref URL_ENCODING: Encoding = Encoding::new(constants::ENCODE_URL);
}

#[derive(Clone)]
pub struct Encoding {
    encode: [u8; 64],
    decode_map: [u8; 256],
    pad_char: Option<u8>,
    strict: bool,
}

impl Encoding {
    pub fn new(encoder: &str) -> Self {
        if encoder.len() != 64 {
            panic!("encoding alphabet is not 64-bytes long")
        }

        if let Some(_) = encoder
            .as_bytes()
            .iter()
            .find(|&&c| c == constants::LF || c == constants::CR)
        {
            panic!("encoding alphabet contains newline character");
        }

        let encode = {
            let mut v = [0u8; 64];
            v.copy_from_slice(encoder.as_bytes());
            v
        };

        let decode_map = {
            let mut v = [0xffu8; 256];

            for (i, &vv) in encode.iter().enumerate() {
                v[vv as usize] = i as u8;
            }

            v
        };

        Self {
            encode,
            decode_map,
            pad_char: Some(constants::STD_PADDING),
            strict: false,
        }
    }

    pub fn decode(&self, dst: &mut [u8], src: &[u8]) -> Result<usize, Error> {
        if src.len() == 0 {
            return Ok(0);
        }

        let mut n = 0usize; // number of bytes written to dst
        let mut src_idx = 0usize;
        while src.len() - src_idx >= 8 && dst.len() - n >= 8 {
            match assemble64(
                self.decode_map[src[src_idx + 0] as usize],
                self.decode_map[src[src_idx + 1] as usize],
                self.decode_map[src[src_idx + 2] as usize],
                self.decode_map[src[src_idx + 3] as usize],
                self.decode_map[src[src_idx + 4] as usize],
                self.decode_map[src[src_idx + 5] as usize],
                self.decode_map[src[src_idx + 6] as usize],
                self.decode_map[src[src_idx + 7] as usize],
            ) {
                Ok(v) => {
                    BigEndian::put_uint64(&mut dst[n..], v);
                    n += 6;
                    src_idx += 8;
                }
                Err(_) => match self.decode_quantum(&mut dst[n..], &src[src_idx..]) {
                    Ok((s, nn)) => {
                        src_idx += s;
                        n += nn;
                    }
                    Err(err) => {
                        return Err(Error::IO(io::Error::new(io::ErrorKind::Other, err), n));
                    }
                },
            }
        }

        while src.len() - src_idx >= 4 && dst.len() - n >= 4 {
            match assemble32(
                self.decode_map[src[src_idx + 0] as usize],
                self.decode_map[src[src_idx + 1] as usize],
                self.decode_map[src[src_idx + 2] as usize],
                self.decode_map[src[src_idx + 3] as usize],
            ) {
                Ok(v) => {
                    BigEndian::put_uint32(&mut dst[n..], v);
                    n += 3;
                    src_idx += 4;
                }
                Err(_) => match self.decode_quantum(&mut dst[n..], &src[src_idx..]) {
                    Ok((s, nn)) => {
                        src_idx += s;
                        n += nn;
                    }
                    Err(err) => {
                        return Err(Error::IO(io::Error::new(io::ErrorKind::Other, err), n));
                    }
                },
            }
        }

        // @dev whether 'if' is ok?
        while src_idx < src.len() {
            match self.decode_quantum(&mut dst[n..], &src[src_idx..]) {
                Ok((s, nn)) => {
                    src_idx += s;
                    n += nn;
                }
                Err(err) => {
                    return Err(Error::IO(io::Error::new(io::ErrorKind::Other, err), n));
                }
            }
        }

        Ok(n)
    }

    pub fn decoded_len(&self, n: usize) -> usize {
        match self.pad_char {
            None => n * 6 / 8,
            _ => n / 4 * 3,
        }
    }

    pub fn decode_string(&self, s: &str) -> Result<Vec<u8>, Error> {
        let mut out = vec![0u8; self.encoded_len(s.len())];
        let n = self.decode(out.as_mut_slice(), s.as_bytes())?;

        out.resize(n, 0);

        Ok(out)
    }

    pub fn encode(&self, dst: &mut [u8], src: &[u8]) {
        if src.len() == 0 {
            return;
        }

        let mut chunks = src.chunks_exact(3);
        let mut dst_idx = 0;
        while let Some(chunk) = chunks.next() {
            let v = ((chunk[0] as usize) << 16) | ((chunk[1] as usize) << 8) | (chunk[2] as usize);

            dst[dst_idx + 0] = self.encode[(v >> 18) & 0x3f];
            dst[dst_idx + 1] = self.encode[(v >> 12) & 0x3f];
            dst[dst_idx + 2] = self.encode[(v >> 6) & 0x3f];
            dst[dst_idx + 3] = self.encode[v & 0x3f];
            dst_idx += 4;
        }

        let remainder = chunks.remainder();
        let val = match remainder.len() {
            1 => (remainder[0] as usize) << 16,
            2 => (remainder[0] as usize) << 16 | (remainder[1] as usize) << 8,
            _ => return,
        };

        dst[dst_idx + 0] = self.encode[(val >> 18) & 0x3f];
        dst[dst_idx + 1] = self.encode[(val >> 12) & 0x3f];

        match remainder.len() {
            2 => {
                dst[dst_idx + 2] = self.encode[(val >> 6) & 0x3f];
                if let Some(c) = self.pad_char {
                    dst[dst_idx + 3] = c;
                }
            }
            1 if self.pad_char.is_some() => {
                let c = self.pad_char.unwrap();

                dst[dst_idx + 2] = c;
                dst[dst_idx + 3] = c;
            }
            _ => {}
        }
    }

    pub fn encoded_len(&self, n: usize) -> usize {
        match self.pad_char {
            None => (n * 8 + 5) / 6,
            _ => (n + 2) / 3 * 4,
        }
    }

    pub fn encode_to_string(&self, src: &[u8]) -> String {
        let mut buf = vec![0u8; self.encoded_len(src.len())];
        self.encode(&mut buf, src);

        String::from_utf8(buf).expect("unfallible")
    }

    pub fn strict(&mut self) -> &Self {
        self.strict = true;
        self
    }

    pub fn with_padding(&mut self, padding: char) -> &Self {
        if !padding.is_ascii() {
            panic!("invalid padding")
        }

        let c = padding as u8;
        if c == constants::CR || c == constants::LF {
            panic!("invalid padding")
        }

        if self.encode.iter().any(|&x| x == c) {
            panic!("padding contained in alphabet");
        }

        self.pad_char = Some(c);

        self
    }

    pub fn without_padding(&mut self) -> &Self {
        self.pad_char = None;

        self
    }

    fn decode_quantum(&self, dst: &mut [u8], src: &[u8]) -> Result<(usize, usize), Error> {
        let mut dbuf = [0u8; 4];

        let mut src_idx = 0usize;
        let mut j = 0usize;
        let dlen = loop {
            if j >= dbuf.len() {
                break j;
            }

            if src.len() == src_idx {
                match j {
                    0 => return Ok((0, 0)),
                    1 => return Err(Error::CorruputInputError(src_idx)),
                    _ if self.pad_char.is_some() => {
                        return Err(Error::CorruputInputError(src_idx - j))
                    }
                    _ => {}
                }

                break j;
            }

            let c = src[src_idx];
            src_idx += 1;
            if self.decode_map[c as usize] != 0xff {
                dbuf[j] = self.decode_map[c as usize];
                j += 1;
                continue;
            }

            if c == constants::LF || c == constants::CR {
                continue;
            }

            if !self.is_pad_char(c) {
                return Err(Error::CorruputInputError(src_idx - 1));
            }

            match j {
                0 | 1 => return Err(Error::CorruputInputError(src_idx - 1)), // at most 2 padding char
                2 => {
                    while src_idx < src.len()
                        && (src[src_idx] == constants::LF || src[src_idx] == constants::CR)
                    {
                        src_idx += 1;
                    }

                    if src_idx == src.len() {
                        return Err(Error::CorruputInputError(src.len()));
                    }

                    if !self.is_pad_char(src[src_idx]) {
                        return Err(Error::CorruputInputError(src_idx - 1));
                    }

                    src_idx += 1;
                }
                _ => {}
            }

            while src_idx < src.len()
                && (src[src_idx] == constants::LF || src[src_idx] == constants::CR)
            {
                src_idx += 1;
            }

            if src_idx < src.len() {
                return Err(Error::CorruputInputError(src_idx));
            }

            break j;
        };

        // Convert 4x 6bit source bytes into 3 bytes
        let val = ((dbuf[0] as u32) << 18)
            | ((dbuf[1] as u32) << 12)
            | ((dbuf[2] as u32) << 6)
            | ((dbuf[3] as u32) << 0);
        dbuf[2] = (val >> 0) as u8;
        dbuf[1] = (val >> 8) as u8;
        dbuf[0] = (val >> 16) as u8;

        // @note dlen cannot be 0 or 1
        if dlen == 4 {
            dst[2] = dbuf[2];
            dbuf[2] = 0;
        }
        if dlen >= 3 {
            dst[1] = dbuf[1];
            if self.strict && dbuf[2] != 0 {
                return Err(Error::CorruputInputError(src_idx - 1));
            }
            dbuf[1] = 0;
        }
        if dlen >= 2 {
            dst[0] = dbuf[0];
            if self.strict && dbuf[2] != 0 {
                return Err(Error::CorruputInputError(src_idx - 2));
            }
        }

        Ok((src_idx, dlen - 1))
    }

    pub fn is_pad_char(&self, c: u8) -> bool {
        match self.pad_char {
            None if c == 0 => true,
            Some(v) if c == v => true,
            _ => false,
        }
    }
}

fn assemble32(n1: u8, n2: u8, n3: u8, n4: u8) -> Result<u32, ()> {
    if n1 | n2 | n3 | n4 == 0xff {
        return Err(());
    }

    let out = ((n1 as u32) << 26) | ((n2 as u32) << 20) | ((n3 as u32) << 14) | ((n4 as u32) << 8);

    return Ok(out);
}

fn assemble64(n1: u8, n2: u8, n3: u8, n4: u8, n5: u8, n6: u8, n7: u8, n8: u8) -> Result<u64, ()> {
    if n1 | n2 | n3 | n4 | n5 | n6 | n7 | n8 == 0xff {
        return Err(());
    }

    let out = (n1 as u64) << 58
        | (n2 as u64) << 52
        | (n3 as u64) << 46
        | (n4 as u64) << 40
        | (n5 as u64) << 34
        | (n6 as u64) << 28
        | (n7 as u64) << 22
        | (n8 as u64) << 16;

    return Ok(out);
}

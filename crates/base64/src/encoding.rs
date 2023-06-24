use crate::{constants, CorruptInputError};
use binary::{BigEndian, ByteOrder};

use lazy_static::lazy_static;

lazy_static! {
    /// RAW_STD_ENCODING is the standard raw, unpadded base64 encoding,
    /// as defined in [RFC 4648 section 3.2].
    /// This is the same as [STD_ENCODING](/encoding/base64/struct.STD_ENCODING.html)
    /// but omits padding characters.
    ///
    /// [RFC 4648 section 3.2]: https://rfc-editor.org/rfc/rfc4648.html#section-3.2
    pub static ref RAW_STD_ENCODING: Encoding = {
        let mut v = Encoding::new(constants::ENCODE_STD);
        v.without_padding();
        v
    };

    /// RAW_URL_ENCODING is the unpadded alternate base64 encoding defined in [RFC 4648].
    /// It is typically used in URLs and file names.
    /// This is the same as [URL_ENCODING](/encoding/base64/struct.URL_ENCODING.html)
    /// but omits padding characters.
    ///
    /// [RFC 4648]: https://rfc-editor.org/rfc/rfc4648.html
    pub static ref RAW_URL_ENCODING: Encoding = {
        let mut v = Encoding::new(constants::ENCODE_URL);
        v.without_padding();
        v
    };

    /// STD_ENCODING is the standard base64 encoding, as defined in
    /// [RFC 4648].
    ///
    /// [RFC 4648]: https://rfc-editor.org/rfc/rfc4648.html
    pub static ref STD_ENCODING: Encoding = Encoding::new(constants::ENCODE_STD);

    /// URL_ENCODING is the alternate base64 encoding defined in [RFC 4648].
    /// It is typically used in URLs and file names.
    ///
    /// [RFC 4648]: https://rfc-editor.org/rfc/rfc4648.html
    pub static ref URL_ENCODING: Encoding = Encoding::new(constants::ENCODE_URL);
}

/// An Encoding is a radix 64 encoding/decoding scheme, defined by a
/// 64-character alphabet. The most common encoding is the "base64"
/// encoding defined in [RFC 4648] and used in MIME ([RFC 2045]) and PEM
/// ([RFC 1421]).  [RFC 4648] also defines an alternate encoding, which is
/// the standard encoding with `-` and `_` substituted for `+` and `/`.
///
/// # Example
/// ```
#[doc = include_str!("../examples/base64.rs")]
/// ```
///
/// [RFC 4648]: https://rfc-editor.org/rfc/rfc4648.html
/// [RFC 2045]: https://rfc-editor.org/rfc/rfc2045.html
/// [RFC 1421]: https://rfc-editor.org/rfc/rfc1421.html
#[derive(Clone, Copy)]
pub struct Encoding {
    encode: [u8; 64],
    decode_map: [u8; 256],
    pub(crate) pad_char: Option<u8>,
    strict: bool,
}

impl Encoding {
    /// Returns a new padded Encoding defined by the given alphabet,
    /// which must be a 64-byte string that does not contain the padding character
    /// or CR / LF ('\r', '\n').
    /// The resulting `Encoding` uses the default padding character ('='),
    /// which may be changed or disabled via [with_padding][Self::with_padding].
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

    /// Decodes `src` using the encoding `self`. It writes at most
    /// [decoded_len(src.len())][Self::decoded_len] bytes to `dst` and returns the number of bytes
    /// written. If `src` contains invalid base64 data, it will return the
    /// number of bytes successfully written or
    /// [CorruptInputError](crate::CorruptInputError).
    /// New line characters (`\r` and `\n`) are ignored.
    ///
    /// # Example
    /// ```
    #[doc = include_str!("../examples/decode.rs")]
    /// ```
    pub fn decode(&self, dst: &mut [u8], src: &[u8]) -> Result<usize, CorruptInputError> {
        if src.len() == 0 {
            return Ok(0);
        }

        let (ndst, nsrc) = (dst.len(), src.len());

        let (mut dst, mut src) = (dst, src);

        let localize_err = |mut err: CorruptInputError, dst_len: usize, src_len: usize| {
            err.idx += nsrc - src_len;
            err.written += ndst - dst_len;
            err
        };

        if usize::BITS == 64 {
            while src.len() >= 8 && dst.len() >= 8 {
                match assemble64(
                    self.decode_map[src[0] as usize],
                    self.decode_map[src[1] as usize],
                    self.decode_map[src[2] as usize],
                    self.decode_map[src[3] as usize],
                    self.decode_map[src[4] as usize],
                    self.decode_map[src[5] as usize],
                    self.decode_map[src[6] as usize],
                    self.decode_map[src[7] as usize],
                ) {
                    Ok(v) => {
                        BigEndian::put_uint64(&mut dst, v);
                        dst = &mut dst[6..];
                        src = &src[8..];
                    }
                    Err(_) => {
                        let (nr, nw) = self
                            .decode_quantum(&mut dst, src)
                            .map_err(|err| localize_err(err, dst.len(), src.len()))?;
                        src = &src[nr..];
                        dst = &mut dst[nw..];
                    }
                }
            }
        }

        while src.len() >= 4 && dst.len() >= 4 {
            match assemble32(
                self.decode_map[src[0] as usize],
                self.decode_map[src[1] as usize],
                self.decode_map[src[2] as usize],
                self.decode_map[src[3] as usize],
            ) {
                Ok(v) => {
                    BigEndian::put_uint32(&mut dst, v);
                    dst = &mut dst[3..];
                    src = &src[4..];
                }
                Err(_) => {
                    let (nr, nw) = self
                        .decode_quantum(&mut dst, src)
                        .map_err(|err| localize_err(err, dst.len(), src.len()))?;
                    src = &src[nr..];
                    dst = &mut dst[nw..];
                }
            }
        }

        // @dev whether 'if' is ok?
        while src.len() > 0 {
            let (nr, nw) = self
                .decode_quantum(&mut dst, src)
                .map_err(|err| localize_err(err, dst.len(), src.len()))?;
            src = &src[nr..];
            dst = &mut dst[nw..];
        }

        Ok(ndst - dst.len())
    }

    /// Returns the maximum length in bytes of the decoded data
    /// corresponding to `n` bytes of base64-encoded data.
    pub fn decoded_len(&self, n: usize) -> usize {
        match self.pad_char {
            None => n * 6 / 8,
            _ => n / 4 * 3, // Padded base64 should always be a multiple of 4 characters in length.
        }
    }

    /// Returns the bytes represented by the base64 string `s`.
    ///
    /// # Example
    /// ```
    #[doc = include_str!("../examples/decode_string.rs")]
    /// ```
    pub fn decode_string(&self, s: &str) -> Result<Vec<u8>, CorruptInputError> {
        let mut out = vec![0u8; self.encoded_len(s.len())];
        let n = self.decode(out.as_mut_slice(), s.as_bytes())?;

        out.resize(n, 0);

        Ok(out)
    }

    /// Encodes `src` using the encoding `self`, writing
    /// [encoded_len(src.len())][Self::encoded_len] bytes to `dst`.
    ///
    /// The encoding pads the output to a multiple of 4 bytes,
    /// so `encode` is not appropriate for use on individual blocks
    /// of a large data stream.
    ///
    /// # Example
    /// ```
    #[doc =include_str!("../examples/encode.rs")]
    /// ```
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

    /// Returns the length in bytes of the base64 encoding
    /// of an input buffer of length `n`.
    pub fn encoded_len(&self, n: usize) -> usize {
        match self.pad_char {
            None => (n * 8 + 5) / 6,
            _ => (n + 2) / 3 * 4,
        }
    }

    /// Returns the base64 encoding of `src`.
    ///
    /// # Example
    /// ```
    #[doc = include_str!("../examples/encode_to_string.rs")]
    /// ```
    pub fn encode_to_string(&self, src: &[u8]) -> String {
        let mut buf = vec![0u8; self.encoded_len(src.len())];
        self.encode(&mut buf, src);

        String::from_utf8(buf).expect("unfallible")
    }

    /// Creates a new encoding identical to enc except with
    /// strict decoding enabled. In this mode, the decoder requires that
    /// trailing padding bits are zero, as described in [RFC 4648 section 3.5].
    ///
    /// Note that the input is still malleable, as new line characters
    /// (CR and LF) are still ignored.
    ///
    /// [RFC 4648 section 3.5]: https://rfc-editor.org/rfc/rfc4648.html#section-3.5
    pub fn strict(&mut self) -> &Self {
        self.strict = true;
        self
    }

    /// Creates a new encoding identical to `self` except
    /// with a specified padding character.
    /// The padding character must not be '\r' or '\n', must not
    /// be contained in the encoding's alphabet and must be a rune equal or
    /// below '\xff'.
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

    /// Creates a new encoding identical to `self` except without padding.
    pub fn without_padding(&mut self) -> &Self {
        self.pad_char = None;

        self
    }

    fn decode_quantum(
        &self,
        dst: &mut [u8],
        src: &[u8],
    ) -> Result<(usize, usize), CorruptInputError> {
        let mut dbuf = [0u8; 4];

        let mut src_idx = 0usize;
        let mut j = 0usize;
        let dlen = loop {
            if j >= dbuf.len() {
                break j;
            }

            if src.len() == src_idx {
                match j {
                    0 => return Ok((src_idx, 0)),
                    1 => return Err(CorruptInputError::new(src, src_idx - 1, 0)),
                    _ if self.pad_char.is_some() => {
                        return Err(CorruptInputError::new(src, src_idx - 2, 0))
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
                return Err(CorruptInputError::new(src, src_idx - 1, 0));
            }

            // We've reached the end and there's padding
            match j {
                0 | 1 => return Err(CorruptInputError::new(src, src_idx - 1, 0)), // at most 2 padding char
                2 => {
                    // "==" is expected, the first "=" is already consumed.
                    // skip over newlines
                    while src_idx < src.len()
                        && (src[src_idx] == constants::LF || src[src_idx] == constants::CR)
                    {
                        src_idx += 1;
                    }

                    if src_idx == src.len() {
                        // not enough padding
                        return Err(CorruptInputError::new(src, src.len(), 0));
                    }

                    if !self.is_pad_char(src[src_idx]) {
                        return Err(CorruptInputError::new(src, src_idx - 1, 0));
                        // incorrect padding
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
                return Err(CorruptInputError::new(src, src_idx, 0));
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
                return Err(CorruptInputError::new(src, src_idx - 1, 0));
            }
            dbuf[1] = 0;
        }
        if dlen >= 2 {
            dst[0] = dbuf[0];
            if self.strict && ((dbuf[1] != 0) || (dbuf[2] != 0)) {
                return Err(CorruptInputError::new(src, src_idx - 2, 0));
            }
        }

        Ok((src_idx, dlen - 1))
    }

    pub fn is_pad_char(&self, c: u8) -> bool {
        match self.pad_char {
            // None if c == 0 => true,
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

//fn new_corrupted_error(idx: usize, written: usize) -> Error {
//    return Error::CorruptInput(CorruptInputError::new(idx, written));
//}

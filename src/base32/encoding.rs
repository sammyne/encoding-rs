use std::{error, io};

use data_encoding::{Encoding as InnerEncoding, Specification};
use lazy_static::lazy_static;

use super::constants;
use crate::Error;

lazy_static! {
    pub static ref STD_ENCODING: Encoding = Encoding::new(constants::ENCODE_STD);
    pub static ref HEX_ENCODING: Encoding = Encoding::new(constants::ENCODE_HEX);
}

#[derive(Clone)]
pub struct Encoding {
    inner: InnerEncoding,
    spec: Specification,
}

impl Encoding {
    pub fn new<T>(encoder: T) -> Self
    where
        T: AsRef<str>,
    {
        let encoder = encoder.as_ref();
        if encoder.len() != 32 {
            panic!("encoding alphabet is not 32-bytes long")
        }

        let mut spec = Specification::new();
        spec.symbols = encoder.to_string();
        spec.padding = Some(constants::STD_PADDING as char);

        Self {
            inner: spec.encoding().unwrap(),
            spec: spec,
        }
    }

    pub fn decode(&self, dst: &mut [u8], src: &[u8]) -> Result<usize, Error> {
        if src.len() == 0 {
            return Ok(0);
        }

        let olen = self.inner.decode_len(src.len()).unwrap();
        let dst = &mut dst[0..olen];

        self.inner
            .decode_mut(src, dst)
            .map_err(|err| new_io_error(format!("{:?}", err), err.written))
    }

    pub fn decoded_len(&self, n: usize) -> usize {
        if self.spec.padding.is_none() {
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
        let olen = self.inner.encode_len(src.len());
        let dst = &mut dst[0..olen];

        self.inner.encode_mut(src, dst);
    }

    pub fn encode_to_string(&self, src: &[u8]) -> String {
        let mut out = String::new();
        self.inner.encode_append(src, &mut out);

        out
    }

    pub fn encoded_len(&self, n: usize) -> usize {
        if self.spec.padding.is_none() {
            (n * 8 + 4) / 5
        } else {
            (n + 4) / 5 * 8
        }
    }

    pub fn is_with_padding(&self) -> bool {
        self.spec.padding.is_some()
    }

    pub fn with_padding(&mut self, padding: char) -> &Self {
        if !padding.is_ascii() {
            panic!("invalid padding")
        }

        let c = padding as u8;
        if c == constants::CR || c == constants::LF {
            panic!("invalid padding")
        }

        if self.spec.symbols.contains(padding) {
            panic!("padding contained in alphabet");
        }

        self.spec.padding = Some(padding);
        self.inner = self.spec.encoding().unwrap();

        self
    }

    pub fn without_padding(&mut self) -> &Self {
        self.spec.padding = None;
        self.inner = self.spec.encoding().unwrap();

        self
    }
}

//fn new_corrupted_error(idx: usize) -> Error {
//    Error::CorruptInputError("base32", idx)
//}

fn new_io_error<E>(err: E, n: usize) -> Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    Error::IO(io::Error::new(io::ErrorKind::Other, err), n)
}

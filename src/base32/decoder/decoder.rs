use std::io::{self, Read};

use crate::base32::decoder::filter::NewlineFilteringReader;
use crate::base32::Encoding;
use crate::builtin;
use crate::errors::Error;

pub struct Decoder<R>
where
    R: Read,
{
    enc: Encoding,
    r: NewlineFilteringReader<R>,

    buf: [u8; 1024],
    nbuf: usize,
    outbuf: [u8; 1024 / 8 * 5],
    outstart: usize,
    outend: usize,
    err: Option<Error>,
    end: bool,
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // Use leftover decoded output from last read.
        if self.outbuf_len() > 0 {
            let n = builtin::copy(buf, self.outbuf_ref_mut());
            self.outstart += n;
            if self.outbuf_len() == 0 {
                return self.error_or(n);
            }
            return Ok(n);
        }

        if self.err.is_some() {
            return self.error_or(0);
        }

        // Read a chunk
        let nn = usize::min(usize::max(buf.len() / 5 * 8, 8), self.buf.len());

        // Minimum amount of bytes that needs to be read each cycle
        let expect_padding = self.enc.pad_char.is_some();
        let min = if expect_padding { 8 - self.nbuf } else { 1 };

        self.nbuf += match read_encoded_data(
            &mut self.r,
            &mut self.buf[self.nbuf..nn],
            min,
            expect_padding,
        ) {
            Ok(n) => n,
            Err(err) => {
                self.err = Some(Error::IO(err, 0));
                0
            }
        };
        if self.nbuf < min {
            return self.error_or(0);
        }

        // Decode chunk into p, or d.out and then p if p is too small.
        let nr = if expect_padding {
            self.nbuf / 8 * 8
        } else {
            self.nbuf
        };
        let nw = self.enc.decoded_len(self.nbuf);

        let (n, end, err) = if nw > buf.len() {
            match self.enc.decode_(&mut self.outbuf, &self.buf[..nr]) {
                Ok((read, end)) => {
                    self.outstart = 0;
                    self.outend = read;
                    let n = builtin::copy(buf, self.outbuf_ref_mut());
                    self.outstart = n;
                    (n, end, None)
                }
                Err(err) => (0, false, Some(err)),
            }
        } else {
            match self.enc.decode_(buf, &self.buf[..nr]) {
                Ok((n, end)) => (n, end, None),
                Err(err) => (0, false, Some(err)),
            }
        };

        self.end = end;

        self.nbuf -= nr;
        for i in 0..self.nbuf {
            self.buf[i] = self.buf[i + nr];
        }
        if err.is_some() && self.err.is_none() {
            self.err = Some(err.unwrap());
        }

        if self.outbuf_len() > 0 {
            // We cannot return all the decoded bytes to the caller in this
            // invocation of Read, so we return a nil error to ensure that Read
            // will be called again.  The error stored in d.err, if any, will be
            // returned with the last set of decoded bytes.
            return Ok(n);
        }

        self.error_or(n)
    }
}

impl<R> Decoder<R>
where
    R: Read,
{
    pub fn new(enc: Encoding, r: R) -> Self {
        Self {
            enc: enc,
            r: NewlineFilteringReader::new(r),
            buf: [0u8; 1024],
            nbuf: 0,
            outbuf: [0u8; 1024 / 8 * 5],
            outstart: 0usize,
            outend: 0usize,
            err: None,
            end: false,
        }
    }

    fn outbuf_ref_mut(&mut self) -> &mut [u8] {
        self.outbuf[self.outstart..self.outend].as_mut()
    }

    fn outbuf_len(&self) -> usize {
        self.outend - self.outstart
    }

    fn error_or<T>(&self, ok: T) -> io::Result<T> {
        match &self.err {
            Some(Error::IO(err, _)) => Err(io::Error::new(err.kind(), err.to_string())),
            Some(err) => Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
            None => Ok(ok),
        }
    }
}

fn read_encoded_data<R>(
    r: &mut R,
    buf: &mut [u8],
    min: usize,
    expect_padding: bool,
) -> Result<usize, io::Error>
where
    R: Read,
{
    let (mut n, mut eof, mut err) = (0usize, false, None);
    while n < min && err.is_none() && !eof {
        match r.read(&mut buf[n..]) {
            Ok(nn) => {
                n += nn;
                eof = nn == 0;
            }
            Err(e) => err = Some(e),
        }
    }

    // data was read, less than min bytes could be read
    if (n < min) && (n > 0) && eof {
        return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
    }

    // no data was read, the buffer already contains some data
    // when padding is disabled this is not an error, as the message can be of
    // any length
    if expect_padding && (min < 8) && (n == 0) && eof {
        return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
    }

    if let Some(err) = err {
        Err(err)
    } else {
        Ok(n)
    }
}

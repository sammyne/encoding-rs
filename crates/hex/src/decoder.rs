use std::io::{self, Read};
use std::ops::Range;

use crate::{Error, BUFFER_SIZE, REVERSE_HEX_TABLE};

struct Decoder<R>
where
    R: Read,
{
    r: R,
    non_io_err: Option<Error>,
    arr: [u8; BUFFER_SIZE],
    arr_range: Range<usize>,
}

impl<R> Read for Decoder<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(err) = self.non_io_err.as_ref() {
            return Err(other_io_error(err.clone()));
        }

        let narr = self.arr_range.end - self.arr_range.start;
        if narr < 2 {
            if narr == 1 {
                self.arr[0] = self.arr[self.arr_range.start];
            }
            self.arr_range.start = 0;
            self.arr_range.end = narr;

            match self.r.read(&mut self.arr[narr..])? {
                0 => {
                    let b = &self.arr[self.arr_range.clone()];
                    if b.len() == 0 {
                        return Ok(0);
                    }

                    let last = b[b.len() - 1];
                    let err = if REVERSE_HEX_TABLE[last as usize] > 0x0f {
                        let err = Error::InvalidByte(last);
                        self.non_io_err = Some(err.clone());
                        other_io_error(err)
                    } else {
                        io::ErrorKind::UnexpectedEof.into()
                    };
                    return Err(err);
                }
                v => {
                    self.arr_range.end += v;
                    if self.arr_range.end - self.arr_range.start < 2 {
                        return Err(io::ErrorKind::Interrupted.into());
                    }
                }
            }
        }

        let b = &self.arr[self.arr_range.clone()];
        let buf = {
            let n = buf.len().min(b.len() / 2);
            &mut buf[..n]
        };

        let decoded_len = match crate::decode(buf, &b[..(buf.len() * 2)]) {
            Ok(v) => v,
            Err((err, 0)) => {
                self.non_io_err = Some(err.clone());
                return Err(other_io_error(err));
            }
            Err((err, decoded_len)) => {
                self.non_io_err = Some(err.clone());
                decoded_len
            }
        };
        self.arr_range.start += decoded_len * 2;

        Ok(decoded_len)
    }
}

/// Returns an [io::Read][std::io::Read] that decodes hexadecimal characters from `r`.
/// `new_decoder` expects that `r` contain only an even number of hexadecimal characters.
pub fn new_decoder<R>(r: R) -> impl Read
where
    R: Read,
{
    Decoder {
        r,
        non_io_err: None,
        arr: [0u8; BUFFER_SIZE],
        arr_range: Range::default(),
    }
}

fn other_io_error<E>(e: E) -> io::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, e)
}

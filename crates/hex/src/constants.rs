use lazy_static::lazy_static;

/// The number of hexadecimal characters to buffer in encoder and decoder.
pub const BUFFER_SIZE: usize = 1024;

lazy_static! {
    pub static ref REVERSE_HEX_TABLE: [u8; 256] = {
        let mut v = [0xff; 256];

        for c in b'0'..=b'9' {
            v[c as usize] = c - b'0';
        }

        for c in b'a'..=b'f' {
            v[c as usize] = c - b'a' + 10;
        }

        for c in b'A'..=b'F' {
            v[c as usize] = c - b'A' + 10;
        }

        v
    };
}

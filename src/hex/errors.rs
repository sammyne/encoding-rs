#[derive(Debug)]
pub enum Error {
    ErrLength(u32),
    InvalidByteError(usize, u8),
}

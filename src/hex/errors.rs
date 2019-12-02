#[derive(Debug)]
pub enum Error {
    InvalidByteError(usize, u8),
    ErrLength(u32),
}

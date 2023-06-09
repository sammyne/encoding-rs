fn main() {
    let src = b"48656c6c6f20476f7068657221";

    let mut dst = vec![0u8; hex::decoded_len(src.len())];
    let n = hex::decode(dst.as_mut_slice(), src).unwrap();

    let got = unsafe { std::str::from_utf8_unchecked(&dst[..n]) };

    const EXPECT: &'static str = "Hello Gopher!";

    assert_eq!(EXPECT, got);
}

fn main() {
    let src = b"Hello Gopher!";

    let mut dst = vec![0u8; hex::encoded_len(src.len())];
    let _ = hex::encode(dst.as_mut_slice(), src);

    const EXPECT: &'static [u8] = b"48656c6c6f20476f7068657221";

    assert_eq!(EXPECT, dst);
}

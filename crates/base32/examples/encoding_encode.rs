use base32::STD_ENCODING;

fn main() {
    let data = b"Hello, world!";
    let mut dst = vec![0u8; STD_ENCODING.encoded_len(data.len())];
    STD_ENCODING.encode(&mut dst, data);

    const EXPECT: &'static str = "JBSWY3DPFQQHO33SNRSCC===";
    let got = String::from_utf8_lossy(dst.as_slice());
    assert_eq!(EXPECT, got.as_ref());
}

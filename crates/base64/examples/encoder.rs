use std::io::Write;

fn main() {
    let input = b"foo\x00bar";

    let mut got = vec![];

    let mut encoder = base64::new_encoder(*base64::STD_ENCODING, &mut got);
    encoder.write_all(input).unwrap();

    // Must flush the encoder when finished to flush any partial blocks.
    // If you comment out the following line and the mem::drop line, the last partial block "r"
    // won't be encoded.
    encoder.flush().unwrap();

    // drop encoder will call encoder.flush() internally.
    std::mem::drop(encoder);

    const EXPECT: &'static [u8] = b"Zm9vAGJhcg==";

    assert_eq!(EXPECT, got);
}

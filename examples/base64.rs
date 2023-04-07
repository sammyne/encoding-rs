use encoding::base64::STD_ENCODING;

fn main() {
    let msg = "Hello, 世界";

    let encoded = STD_ENCODING.encode_to_string(msg.as_bytes());

    const ENCODED_EXPECT: &'static str = "SGVsbG8sIOS4lueVjA==";
    assert_eq!(ENCODED_EXPECT, encoded);

    let decoded = STD_ENCODING.decode_string(&encoded).expect("decode");
    let decoded_str = String::from_utf8_lossy(&decoded);

    assert_eq!(decoded_str.as_ref(), msg);
}

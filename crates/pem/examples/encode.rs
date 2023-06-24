use std::collections::HashMap;

use pem::Block;

fn main() {
    let block = {
        let mut headers = HashMap::default();
        headers.insert("Animal".to_string(), "Gopher".to_string());

        Block {
            type_: "MESSAGE".to_string(),
            headers,
            bytes: b"test".to_vec(),
        }
    };

    let mut buf = Vec::default();
    pem::encode(&mut buf, &block).unwrap();

    const EXPECT: &'static str = r#"-----BEGIN MESSAGE-----
Animal: Gopher

dGVzdA==
-----END MESSAGE-----
"#;

    let got = String::from_utf8_lossy(buf.as_slice());
    assert_eq!(EXPECT, got);
}

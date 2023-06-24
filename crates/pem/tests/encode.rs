use std::collections::HashMap;

use pem::Block;

#[test]
fn bad_encode() {
    let b = {
        let mut headers = HashMap::default();
        headers.insert("X:Y".to_string(), "Z".to_string());

        Block {
            type_: "BAD".to_string(),
            headers,
            ..Default::default()
        }
    };

    let mut buf = vec![];
    pem::encode(&mut buf, &b).expect_err("encode did not report invalid header");

    assert_eq!(
        0,
        buf.len(),
        "encode wrote data before reporting invalid header"
    );

    assert!(
        pem::encode_to_memory(&b).is_none(),
        "encode_to_memory returned non-nil data"
    );
}

#[test]
fn encode() {
    let r = pem::encode_to_memory(&testbot::PRIVATE_KEY2)
        .map(|v| unsafe { String::from_utf8_unchecked(v) })
        .unwrap();

    assert_eq!(*testbot::PEM_PRIVATE_KEY2, r);
}

mod testbot;

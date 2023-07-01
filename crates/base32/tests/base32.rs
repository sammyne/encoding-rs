use std::io::{Read, Write};

use base32::STD_ENCODING;

#[test]
fn big() {
    const ALPHA: &'static [u8] =
        "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

    let n = 3 * 1000 + 1;
    let mut raw = vec![0u8; n];
    for i in 0..n {
        raw[i] = ALPHA[i % ALPHA.len()];
    }

    let mut encoded = vec![];
    let mut w = base32::new_encoder(*STD_ENCODING, &mut encoded);
    let nn = w.write(&raw).unwrap();
    assert_eq!(n, nn);

    w.flush().unwrap();
    std::mem::drop(w);

    let mut decoded = vec![];
    base32::new_decoder(STD_ENCODING.clone(), encoded.as_slice())
        .read_to_end(&mut decoded)
        .unwrap();

    if raw != decoded {
        let mut i = 0usize;
        while i < raw.len() && i < decoded.len() {
            if decoded[i] != raw[i] {
                break;
            }
            i += 1;
        }
        panic!("Decode(Encode({}-byte string)) failed at offset {}", n, i);
    }
}

#[test]
fn encoded_decoded_len() {
    struct Case {
        in_: usize,
        want_enc: usize,
        want_dec: usize,
    }
    let new_case = |in_, want_enc, want_dec| Case {
        in_,
        want_enc,
        want_dec,
    };
    let data = [b'x'; 100];

    let test_vector = vec![
        (
            STD_ENCODING.clone(),
            vec![
                new_case(0, 0, 0),
                new_case(1, 8, 5),
                new_case(5, 8, 5),
                new_case(6, 16, 10),
                new_case(10, 16, 10),
            ],
        ),
        (
            STD_ENCODING.clone().with_padding(None).clone(),
            vec![
                new_case(0, 0, 0),
                new_case(1, 2, 1),
                new_case(2, 4, 2),
                new_case(5, 8, 5),
                new_case(6, 10, 6),
                new_case(7, 12, 7),
                new_case(10, 16, 10),
                new_case(11, 18, 11),
            ],
        ),
    ];

    for (i, (enc, cases)) in test_vector.iter().enumerate() {
        for (j, w) in cases.iter().enumerate() {
            let enc_len = enc.encoded_len(w.in_);
            let dec_len = enc.decoded_len(enc_len);
            let enc = enc.encode_to_string(&data[..w.in_]);

            assert_eq!(
                enc.len(),
                enc_len,
                "#{}-#{} encoded_len({})={}, but got {}",
                i,
                j,
                w.in_,
                &enc_len,
                enc
            );
            assert_eq!(w.want_enc, enc_len, "#{}-#{} bad encoded length", i, j);
            assert_eq!(w.want_dec, dec_len, "#{}-#{} bad decoded length", i, j);
        }
    }
}

use base64::{Encoding, RAW_STD_ENCODING, STD_ENCODING};

mod testbot;

#[test]
fn encode() {
    let test_vector = &testbot::ENCODING_TESTS;
    for (i, p) in testbot::PAIRS.iter().enumerate() {
        // non-strict ones
        for tt in test_vector.iter() {
            let got = tt.enc.encode_to_string(p.decoded);
            let expect = (tt.conv)(p.encoded);
            assert_eq!(
                expect, got,
                "#{} non-strict encode({}) failed",
                i, p.escape_ascii_decoded
            );
        }

        // strict ones
        for tt in test_vector.iter() {
            let enc = {
                let mut enc = tt.enc.clone();
                enc.strict();
                enc
            };

            let got = enc.encode_to_string(p.decoded);
            let expect = (tt.conv)(p.encoded);
            assert_eq!(
                expect, got,
                "#{} strict encode({}) failed",
                i, p.escape_ascii_decoded
            );
        }
    }
}

#[test]
fn encoded_len() {
    struct Case {
        enc: Encoding,
        n: usize,
        want: usize,
    }

    let new_case = |enc: Encoding, n: usize, want: usize| -> Case { Case { enc, n, want } };

    let test_vector: Vec<Case> = vec![
        new_case(*RAW_STD_ENCODING, 0, 0),
        new_case(*RAW_STD_ENCODING, 1, 2),
        new_case(*RAW_STD_ENCODING, 2, 3),
        new_case(*RAW_STD_ENCODING, 3, 4),
        new_case(*RAW_STD_ENCODING, 7, 10),
        new_case(*STD_ENCODING, 0, 0),
        new_case(*STD_ENCODING, 1, 4),
        new_case(*STD_ENCODING, 2, 4),
        new_case(*STD_ENCODING, 3, 4),
        new_case(*STD_ENCODING, 4, 8),
        new_case(*STD_ENCODING, 7, 12),
    ];

    for tt in test_vector {
        let got = tt.enc.encoded_len(tt.n);
        assert_eq!(tt.want, got, "encoded_len({})", tt.n);
    }
}

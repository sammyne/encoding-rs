use std::{
    io::{self, Read},
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use base64::{Encoding, RAW_STD_ENCODING, STD_ENCODING};

mod testbot;

#[test]
fn decode() {
    for (i, p) in testbot::PAIRS.iter().enumerate() {
        for tt in testbot::ENCODING_TESTS.iter() {
            let encoded = (tt.conv)(p.encoded);

            {
                let mut dbuf = vec![0u8; tt.enc.decoded_len(encoded.len())];
                let n = tt
                    .enc
                    .decode(dbuf.as_mut_slice(), encoded.as_bytes())
                    .expect("unexpected error");
                assert_eq!(
                    p.decoded.len(),
                    n,
                    "#{} decode({}): expect length {}, got {}",
                    i,
                    encoded,
                    p.decoded.len(),
                    n
                );

                let got = &dbuf[..n];
                assert_eq!(p.decoded, got, "#{} decode({})", i, encoded);
            }

            {
                let got = tt.enc.decode_string(&encoded).expect("unexpected error");
                assert_eq!(p.decoded, got, "#{} decode_string({})", i, encoded);
            }
        }
    }
}

#[test]
fn decode_bounds() {
    let mut buf = [0u8; 32];
    let s = STD_ENCODING.encode_to_string(&buf);

    let n = STD_ENCODING.decode(&mut buf, s.as_bytes()).expect("decode");
    assert_eq!(buf.len(), n, "STD_ENCODING.decode");
}

#[test]
fn decode_corrupt() {
    struct Case {
        input: String,
        offset: Option<usize>, // none means no corruption
    }

    impl Case {
        fn new(input: &str, offset: isize) -> Self {
            let offset = if offset < 0 {
                None
            } else {
                Some(offset as usize)
            };
            Self {
                input: input.to_string(),
                offset,
            }
        }
    }

    let test_vector: Vec<Case> = vec![
        ("", -1),
        ("\n", -1),
        ("AAA=\n", -1),
        ("AAAA\n", -1),
        ("!!!!", 0),
        ("====", 0),
        ("x===", 1),
        ("=AAA", 0),
        ("A=AA", 1),
        ("AA=A", 2),
        ("AA==A", 4),
        ("AAA=AAAA", 4),
        ("AAAAA", 4),
        ("AAAAAA", 4),
        ("A=", 1),
        ("A==", 1),
        ("AA=", 3),
        ("AA==", -1),
        ("AAA=", -1),
        ("AAAA", -1),
        ("AAAAAA=", 7),
        ("YWJjZA=====", 8),
        ("A!\n", 1),
        ("A=\n", 1),
    ]
    .into_iter()
    .map(|(input, offset)| Case::new(input, offset))
    .collect();

    for tc in test_vector {
        let mut dbuf = vec![0u8; STD_ENCODING.decoded_len(tc.input.len())];
        match STD_ENCODING.decode(dbuf.as_mut_slice(), tc.input.as_bytes()) {
            Ok(_) if tc.offset.is_some() => panic!(
                "decoder failed to detect corruption in {} at offset {}",
                tc.input,
                tc.offset.unwrap()
            ),
            Ok(_) => {}
            Err(_) if tc.offset.is_none() => {
                panic!("decoder wrongly detected corruption in {}", tc.input)
            }
            Err(err) => assert_eq!(err.idx, tc.offset.unwrap(), "corruption in {}", tc.input),
        }
    }
}

#[test]
fn decoded_len() {
    struct Case {
        enc: Encoding,
        n: usize,
        want: usize,
    }

    let new_case = |enc: Encoding, n: usize, want: usize| -> Case { Case { enc, n, want } };

    let test_vector = vec![
        new_case(*RAW_STD_ENCODING, 0, 0),
        new_case(*RAW_STD_ENCODING, 2, 1),
        new_case(*RAW_STD_ENCODING, 3, 2),
        new_case(*RAW_STD_ENCODING, 4, 3),
        new_case(*RAW_STD_ENCODING, 10, 7),
        new_case(*STD_ENCODING, 0, 0),
        new_case(*STD_ENCODING, 4, 3),
        new_case(*STD_ENCODING, 8, 6),
    ];

    for tt in test_vector {
        let got = tt.enc.decoded_len(tt.n);
        assert_eq!(tt.want, got, "decoded_len({})", tt.n);
    }
}

/// https://github.com/golang/go/issues/15656
#[test]
fn golang_decoder_issue15656() {
    let err = STD_ENCODING
        .clone()
        .strict()
        .decode_string("WvLTlMrX9NpYDQlEIFlnDB==")
        .expect_err("1st decode");
    assert_eq!(22, err.idx, "1st decode got wrong err");

    let _ = STD_ENCODING
        .clone()
        .strict()
        .decode_string("WvLTlMrX9NpYDQlEIFlnDA==")
        .expect("2nd decode");

    let _ = STD_ENCODING
        .decode_string("WvLTlMrX9NpYDQlEIFlnDB==")
        .expect("3rd decode");
}

/// tests that we don't ignore errors from our underlying reader https://github.com/golang/go/issues/3577
#[test]
fn golang_decoder_issue3577() {
    let (tx, rx) = mpsc::channel();

    let want_err = "my error";

    tx.send(NextRead::new(5, None)).expect("send 1st NextRead");
    tx.send(NextRead::new(10, Some(want_err)))
        .expect("send 2nd NextRead");
    tx.send(NextRead::new(0, Some(want_err)))
        .expect("send 3rd NextRead");

    let mut d = {
        let r = FaultInjectReader {
            source: b"VHdhcyBicmlsbGlnLCBhbmQgdGhlIHNsaXRoeSB0b3Zlcw==".to_vec(), // twas brillig...
            nextc: rx,
        };
        base64::new_decoder(*STD_ENCODING, r)
    };

    thread::sleep(Duration::from_secs(3));

    let mut buf = vec![];
    let err = d.read_to_end(&mut buf).expect_err("read");

    assert_eq!(io::ErrorKind::Other, err.kind(), "bad error kind");

    let got = err.into_inner().unwrap().to_string();
    assert_eq!(want_err, &got, "bad error");
}

/// https://github.com/golang/go/issues/4779
#[test]
fn golang_decoder_issue4779() {
    const ENCODED: &'static str = r#"CP/EAT8AAAEF
AQEBAQEBAAAAAAAAAAMAAQIEBQYHCAkKCwEAAQUBAQEBAQEAAAAAAAAAAQACAwQFBgcICQoLEAAB
BAEDAgQCBQcGCAUDDDMBAAIRAwQhEjEFQVFhEyJxgTIGFJGhsUIjJBVSwWIzNHKC0UMHJZJT8OHx
Y3M1FqKygyZEk1RkRcKjdDYX0lXiZfKzhMPTdePzRieUpIW0lcTU5PSltcXV5fVWZnaGlqa2xtbm
9jdHV2d3h5ent8fX5/cRAAICAQIEBAMEBQYHBwYFNQEAAhEDITESBEFRYXEiEwUygZEUobFCI8FS
0fAzJGLhcoKSQ1MVY3M08SUGFqKygwcmNcLSRJNUoxdkRVU2dGXi8rOEw9N14/NGlKSFtJXE1OT0
pbXF1eX1VmZ2hpamtsbW5vYnN0dXZ3eHl6e3x//aAAwDAQACEQMRAD8A9VSSSSUpJJJJSkkkJ+Tj
1kiy1jCJJDnAcCTykpKkuQ6p/jN6FgmxlNduXawwAzaGH+V6jn/R/wCt71zdn+N/qL3kVYFNYB4N
ji6PDVjWpKp9TSXnvTf8bFNjg3qOEa2n6VlLpj/rT/pf567DpX1i6L1hs9Py67X8mqdtg/rUWbbf
+gkp0kkkklKSSSSUpJJJJT//0PVUkkklKVLq3WMDpGI7KzrNjADtYNXvI/Mqr/Pd/q9W3vaxjnvM
NaCXE9gNSvGPrf8AWS3qmba5jjsJhoB0DAf0NDf6sevf+/lf8Hj0JJATfWT6/dV6oXU1uOLQeKKn
EQP+Hubtfe/+R7Mf/g7f5xcocp++Z11JMCJPgFBxOg7/AOuqDx8I/ikpkXkmSdU8mJIJA/O8EMAy
j+mSARB/17pKVXYWHXjsj7yIex0PadzXMO1zT5KHoNA3HT8ietoGhgjsfA+CSnvvqh/jJtqsrwOv
2b6NGNzXfTYexzJ+nU7/ALkf4P8Awv6P9KvTQQ4AgyDqCF85Pho3CTB7eHwXoH+LT65uZbX9X+o2
bqbPb06551Y4
"#;

    let encoded_short = ENCODED.replace('\n', "");

    let mut res1 = vec![];
    let _ = base64::new_decoder(*STD_ENCODING, ENCODED.as_bytes())
        .read_to_end(&mut res1)
        .expect("decode encoded");

    let mut res2 = vec![];
    let _ = base64::new_decoder(*STD_ENCODING, encoded_short.as_bytes())
        .read_to_end(&mut res2)
        .expect("decode encoded_short");

    assert_eq!(res1, res2);
}

/// https://github.com/golang/go/issues/7733
#[test]
fn golang_decoder_issue7733() {
    match STD_ENCODING.decode_string("YWJjZA=====") {
        Ok(_) => panic!("shouldn't produce decoded"),
        Err(err) => assert_eq!(8, err.idx),
    }
}

#[test]
fn new_line_characters() {
    // Each of these should decode to the string "sure", without errors.
    const EXPECTED: &'static [u8] = b"sure";

    let examples = vec![
        "c3VyZQ==",
        "c3VyZQ==\r",
        "c3VyZQ==\n",
        "c3VyZQ==\r\n",
        "c3VyZ\r\nQ==",
        "c3V\ryZ\nQ==",
        "c3V\nyZ\rQ==",
        "c3VyZ\nQ==",
        "c3VyZQ\n==",
        "c3VyZQ=\n=",
        "c3VyZQ=\r\n\r\n=",
    ];

    for e in examples {
        let buf = STD_ENCODING.decode_string(e).expect(&format!("decode {e}"));
        assert_eq!(EXPECTED, buf, "decode {e}");
    }
}

struct FaultInjectReader {
    source: Vec<u8>,
    nextc: Receiver<NextRead>,
}

struct NextRead {
    n: usize,
    err: Option<String>,
}

impl NextRead {
    fn new(n: usize, err: Option<&str>) -> Self {
        Self {
            n,
            err: err.map(|v| v.to_string()),
        }
    }
}

impl io::Read for FaultInjectReader {
    fn read(&mut self, p: &mut [u8]) -> io::Result<usize> {
        let nr = self.nextc.recv().expect("try to recv next read");

        let ell = p.len();
        let p = &mut p[..nr.n.min(ell)];
        let n = builtin::copy(p, self.source.as_slice());

        self.source = self.source[n..].to_vec();

        if let Some(err) = &nr.err {
            Err(io::Error::new(io::ErrorKind::Other, err.clone()))
        } else {
            Ok(n)
        }
    }
}

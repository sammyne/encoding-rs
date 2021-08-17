use std::io::{self, ErrorKind, Read, Write};

use crate::base32::{Decoder, Encoder, STD_ENCODING};
use crate::{builtin, Error};

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
    let mut w = Encoder::new(STD_ENCODING.clone(), &mut encoded);
    let nn = w.write(&raw).unwrap();
    assert_eq!(n, nn);

    w.flush().unwrap();

    let mut decoded = vec![];
    Decoder::new(STD_ENCODING.clone(), encoded.as_slice())
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
fn buffered_decoding_same_error() {
    struct Case<'a> {
        prefix: &'a str,
        chunk_combinations: Vec<Vec<&'a str>>,
        err: Option<ErrorKind>,
    }
    let new_case = |prefix, chunk_combinations, err| Case {
        prefix,
        chunk_combinations,
        err,
    };

    let test_vector = vec![
        // NBSWY3DPO5XXE3DE == helloworld
        // Test with "ZZ" as extra input
        new_case(
            "helloworld",
            vec![
                vec!["NBSW", "Y3DP", "O5XX", "E3DE", "ZZ"],
                vec!["NBSWY3DPO5XXE3DE", "ZZ"],
                vec!["NBSWY3DPO5XXE3DEZZ"],
                vec!["NBS", "WY3", "DPO", "5XX", "E3D", "EZZ"],
                vec!["NBSWY3DPO5XXE3", "DEZZ"],
            ],
            Some(ErrorKind::UnexpectedEof),
        ),
        // Test with "ZZY" as extra input
        new_case(
            "helloworld",
            vec![
                vec!["NBSW", "Y3DP", "O5XX", "E3DE", "ZZY"],
                vec!["NBSWY3DPO5XXE3DE", "ZZY"],
                vec!["NBSWY3DPO5XXE3DEZZY"],
                vec!["NBS", "WY3", "DPO", "5XX", "E3D", "EZZY"],
                vec!["NBSWY3DPO5XXE3", "DEZZY"],
            ],
            Some(ErrorKind::UnexpectedEof),
        ),
        // Normal case, this is valid input
        new_case(
            "helloworld",
            vec![
                vec!["NBSW", "Y3DP", "O5XX", "E3DE"],
                vec!["NBSWY3DPO5XXE3DE"],
                vec!["NBS", "WY3", "DPO", "5XX", "E3D", "E"],
                vec!["NBSWY3DPO5XXE3", "DE"],
            ],
            None,
        ),
        // MZXW6YTB = fooba
        new_case(
            "fooba",
            vec![
                vec!["MZXW6YTBZZ"],
                vec!["MZXW6YTBZ", "Z"],
                vec!["MZXW6YTB", "ZZ"],
                vec!["MZXW6YT", "BZZ"],
                vec!["MZXW6Y", "TBZZ"],
                vec!["MZXW6Y", "TB", "ZZ"],
                vec!["MZXW6", "YTBZZ"],
                vec!["MZXW6", "YTB", "ZZ"],
                vec!["MZXW6", "YT", "BZZ"],
            ],
            Some(ErrorKind::UnexpectedEof),
        ),
        // Normal case, this is valid input
        new_case(
            "fooba",
            vec![
                vec!["MZXW6YTB"],
                vec!["MZXW6YT", "B"],
                vec!["MZXW6Y", "TB"],
                vec!["MZXW6", "YTB"],
                vec!["MZXW6", "YT", "B"],
                vec!["MZXW", "6YTB"],
                vec!["MZXW", "6Y", "TB"],
            ],
            None,
        ),
    ];

    for (i, v) in test_vector.iter().enumerate() {
        for (j, w) in v.chunk_combinations.iter().enumerate() {
            let r = {
                let mut r: Box<dyn Read> = Box::new(w[0].as_bytes());
                for i in 1..w.len() {
                    r = Box::new(r.chain(w[i].as_bytes()));
                }
                r
            };

            let mut decoder = Decoder::new(STD_ENCODING.clone(), r);

            let mut buf = vec![];
            let status = decoder.read_to_end(&mut buf);
            assert_eq!(
                status.is_err(),
                v.err.is_some(),
                "#{}-#{} unexpected error for '{}'",
                i,
                j,
                v.prefix,
            );
            if let Err(err) = status {
                assert_eq!(
                    v.err.unwrap(),
                    err.kind(),
                    "#{}-#{} unexpected error kind for '{}'",
                    i,
                    j,
                    v.prefix,
                );
            }
        }
    }
}

#[test]
fn decode() {
    let pairs = pairs();
    for (i, v) in pairs.iter().enumerate() {
        let mut got = "-".repeat(STD_ENCODING.decoded_len(v.encoded.len()));
        let n = STD_ENCODING
            .decode(unsafe { got.as_bytes_mut() }, v.encoded.as_bytes())
            .unwrap();
        got.truncate(n);
        assert_eq!(v.decoded, got, "#{}", i);
    }
}

#[test]
fn decode_corrupt() {
    struct TestCase {
        input: &'static str,
        offset: Option<usize>,
    }
    let new_case = |input, offset: i32| {
        let offset = if -1 == offset {
            None
        } else {
            Some(offset as usize)
        };
        TestCase { input, offset }
    };

    let test_vector = &[
        new_case("", -1),
        new_case("!!!!", 0),
        new_case("x===", 0),
        new_case("AA=A====", 2),
        new_case("AAA=AAAA", 3),
        new_case("MMMMMMMMM", 8),
        new_case("MMMMMM", 0),
        new_case("A=", 1),
        new_case("AA=", 3),
        new_case("AA==", 4),
        new_case("AA===", 5),
        new_case("AAAA=", 5),
        new_case("AAAA==", 6),
        new_case("AAAAA=", 6),
        new_case("AAAAA==", 7),
        new_case("A=======", 1),
        new_case("AA======", -1),
        new_case("AAA=====", 3),
        new_case("AAAA====", -1),
        new_case("AAAAA===", -1),
        new_case("AAAAAA==", 6),
        new_case("AAAAAAA=", -1),
        new_case("AAAAAAAA", -1),
    ];

    for (i, tc) in test_vector.iter().enumerate() {
        let mut dbuf = vec![0u8; STD_ENCODING.decoded_len(tc.input.len())];
        let result = STD_ENCODING.decode(&mut dbuf, tc.input.as_bytes());
        if tc.offset.is_none() {
            assert!(
                result.is_ok(),
                "#{} Decoder wrongly detected corruption in {}",
                i,
                tc.input
            );
            continue;
        }

        let idx = tc.offset.unwrap();
        match result.unwrap_err() {
            Error::CorruptInputError(_, n) => {
                assert_eq!(idx, n, "#{} wrong corruption in {}", i, tc.input)
            }
            _ => {}
        }
    }
}

#[test]
fn decode_read_all() {
    let encodings = [
        STD_ENCODING.clone(),
        STD_ENCODING.clone().with_padding(None).clone(),
    ];

    for (i, pair) in pairs().iter().enumerate() {
        for (j, encoding) in encodings.iter().enumerate() {
            let encoded = if encoding.pad_char.is_some() {
                pair.encoded.to_string()
            } else {
                pair.encoded.replace('=', "")
            };

            let mut dec_reader = vec![];
            Decoder::new(encoding.clone(), encoded.as_bytes())
                .read_to_end(&mut dec_reader)
                .unwrap();
            assert_eq!(
                pair.decoded.as_bytes(),
                dec_reader.as_slice(),
                "#{}-#{}",
                i,
                j
            );
        }
    }
}

#[test]
fn decode_small_buffer() {
    let encodings = [
        STD_ENCODING.clone(),
        STD_ENCODING.clone().with_padding(None).clone(),
    ];

    for bs in 1..200 {
        for (i, pair) in pairs().iter().enumerate() {
            for (j, encoding) in encodings.iter().enumerate() {
                let encoded = if encoding.pad_char.is_some() {
                    pair.encoded.to_string()
                } else {
                    pair.encoded.replace('=', "")
                };

                let mut decoder = Decoder::new(encoding.clone(), encoded.as_bytes());

                let mut all_read = vec![];
                loop {
                    let mut buf = vec![0u8; bs];
                    let n = decoder.read(&mut buf).unwrap();
                    all_read.extend_from_slice(&buf[..n]);
                    if n == 0 {
                        break;
                    }
                }

                let got = String::from_utf8_lossy(&all_read).to_string();
                assert_eq!(
                    pair.decoded, got,
                    "#{}-#{} decoded failed for buffer size={}",
                    i, j, bs
                );
            }
        }
    }
}

#[test]
fn decode_string() {
    for (i, v) in pairs().iter().enumerate() {
        let got = STD_ENCODING.decode_string(v.encoded).unwrap();
        assert_eq!(v.decoded.as_bytes(), got.as_slice(), "#{}", i);
    }
}

#[test]
fn decode_with_padding() {
    let encodings = vec![
        STD_ENCODING.clone(),
        STD_ENCODING.clone().with_padding(Some(b'-')).clone(),
        STD_ENCODING.clone().with_padding(None).clone(),
    ];

    for (i, enc) in encodings.iter().enumerate() {
        for (j, pair) in pairs().iter().enumerate() {
            let input = pair.decoded;
            let encoded = enc.encode_to_string(input.as_bytes());

            let decoded = enc.decode_string(encoded.as_str()).unwrap();
            let got = String::from_utf8_lossy(&decoded).to_string();
            assert_eq!(input, got, "#{}-#{} unexpected decoded result", i, j);
        }
    }
}

#[test]
fn decode_with_wrong_padding() {
    let encoded = STD_ENCODING.encode_to_string("foobar".as_bytes());

    STD_ENCODING
        .clone()
        .with_padding(Some(b'-'))
        .decode_string(encoded.as_str())
        .expect_err("expected error due to bad padding");

    STD_ENCODING
        .clone()
        .with_padding(None)
        .decode_string(encoded.as_str())
        .expect_err("expected error due to no padding");
}

#[test]
fn decoder() {
    for (i, v) in pairs().iter().enumerate() {
        let mut decoder = Decoder::new(STD_ENCODING.clone(), v.encoded.as_bytes());
        let mut dbuf = vec![0u8; STD_ENCODING.decoded_len(v.encoded.len())];
        let count = decoder.read(&mut dbuf).unwrap();
        assert_eq!(v.decoded.len(), count, "#{} Read from {}", i, v.encoded);

        let got = String::from_utf8_lossy(&dbuf[..count]).to_string();
        assert_eq!(v.decoded, got, "#{} decode of {}", i, v.encoded);

        if count != 0 {
            let count = decoder.read(&mut dbuf).unwrap();
            assert_eq!(0, count, "#{} 2nd read should trigger EOF", i);
        }
    }
}

#[test]
fn decoder_buffering() {
    let bigtest = bigtest();
    for bs in 1..=12 {
        let mut decoder = Decoder::new(STD_ENCODING.clone(), bigtest.encoded.as_bytes());
        let mut buf = vec![0u8; bigtest.decoded.len() + 12];

        let mut total = 0usize;
        let mut eof = false;
        while !eof && total < bigtest.decoded.len() {
            let n = decoder.read(&mut buf[total..(total + bs)]).unwrap();
            total += n;
            eof = n == 0;
        }

        let got = String::from_utf8_lossy(&buf[..total]).to_string();
        assert_eq!(
            bigtest.decoded, got,
            "Decoding/{} of {}",
            bs, bigtest.encoded
        );
    }
}

#[test]
fn decoder_error() {
    const INPUT: &'static str = "MZXW6YTb";
    let mut dbuf = vec![0u8; STD_ENCODING.decoded_len(INPUT.len())];
    let br = BadReader {
        data: INPUT.as_bytes().to_vec(),
        ..Default::default()
    };

    let mut decoder = Decoder::new(STD_ENCODING.clone(), br);
    match decoder.read(&mut dbuf) {
        Ok(_) => panic!("expect error"),
        Err(err) => {
            assert_eq!(ErrorKind::Other, err.kind(), "unexpected error kind");
            assert_eq!(
                true,
                err.to_string()
                    .contains("illegal base32 data at input byte 7"),
                "unexpected error msg"
            );
        }
    }
}

#[test]
fn decoder_issue4779() {
    const ENCODED: &'static str = concat!(
        "JRXXEZLNEBUXA43VNUQGI33MN5ZCA43JOQQGC3LFOQWCAY3PNZZWKY3UMV2HK4\n",
        "RAMFSGS4DJONUWG2LOM4QGK3DJOQWCA43FMQQGI3YKMVUXK43NN5SCA5DFNVYG64RANFXGG2LENFSH\n",
        "K3TUEB2XIIDMMFRG64TFEBSXIIDEN5WG64TFEBWWCZ3OMEQGC3DJOF2WCLRAKV2CAZLONFWQUYLEEB\n",
        "WWS3TJNUQHMZLONFQW2LBAOF2WS4ZANZXXG5DSOVSCAZLYMVZGG2LUMF2GS33OEB2WY3DBNVRW6IDM\n",
        "MFRG64TJOMQG42LTNEQHK5AKMFWGS4LVNFYCAZLYEBSWCIDDN5WW233EN4QGG33OONSXC5LBOQXCAR\n",
        "DVNFZSAYLVORSSA2LSOVZGKIDEN5WG64RANFXAU4TFOBZGK2DFNZSGK4TJOQQGS3RAOZXWY5LQORQX\n",
        "IZJAOZSWY2LUEBSXG43FEBRWS3DMOVWSAZDPNRXXEZJAMV2SAZTVM5UWC5BANZ2WY3DBBJYGC4TJMF\n",
        "2HK4ROEBCXQY3FOB2GK5LSEBZWS3TUEBXWGY3BMVRWC5BAMN2XA2LEMF2GC5BANZXW4IDQOJXWSZDF\n",
        "NZ2CYIDTOVXHIIDJNYFGG5LMOBQSA4LVNEQG6ZTGNFRWSYJAMRSXGZLSOVXHIIDNN5WGY2LUEBQW42\n",
        "LNEBUWIIDFON2CA3DBMJXXE5LNFY==\n",
        "====",
    );
    let encoded_short = ENCODED.replace("\n", "");

    let mut res1 = vec![];
    Decoder::new(STD_ENCODING.clone(), ENCODED.as_bytes())
        .read_to_end(&mut res1)
        .unwrap();

    let mut res2 = vec![];
    Decoder::new(STD_ENCODING.clone(), encoded_short.as_bytes())
        .read_to_end(&mut res2)
        .unwrap();

    assert_eq!(res1, res2, "Decoded results not equal");
}

#[test]
fn encode() {
    let pairs = pairs();
    for (i, v) in pairs.iter().enumerate() {
        let got = STD_ENCODING.encode_to_string(v.decoded.as_bytes());
        assert_eq!(v.encoded, got, "#{}", i);
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

#[test]
fn encoder() {
    for (i, p) in pairs().iter().enumerate() {
        let mut bb = vec![];
        let mut encoder = Encoder::new(STD_ENCODING.clone(), &mut bb);
        let _ = encoder.write(p.decoded.as_bytes());
        let _ = encoder.flush();

        let got = String::from_utf8_lossy(bb.as_slice()).to_string();
        assert_eq!(p.encoded, got, "#{} encode({})", i, p.decoded);
    }
}

#[test]
fn encoder_buffering() {
    let bigtest = bigtest();
    let input = bigtest.decoded.as_bytes();
    for bs in 1..=12 {
        let mut bb = vec![];
        let mut encoder = Encoder::new(STD_ENCODING.clone(), &mut bb);
        for pos in (0..input.len()).step_by(bs) {
            let end = usize::min(pos + bs, input.len());
            let n = encoder.write(input[pos..end].as_ref()).unwrap();
            assert_eq!(
                end - pos,
                n,
                "buffer size {} gave wrong length at offset {}",
                bs,
                pos
            )
        }
        encoder.flush().unwrap();

        let got = String::from_utf8_lossy(bb.as_slice()).to_string();
        assert_eq!(
            bigtest.encoded, got,
            "Encoding/{} of {}",
            bs, bigtest.decoded
        );
    }
}

/*
#[test]
fn issue20044() {
    let fake_bad_error = || Some(io::Error::new(ErrorKind::Other, "bad reader error"));

    struct TestCase {
        r: BadReader,
        res: String,
        err: Option<io::Error>,
        dbuflen: usize,
    }
    let new_test_case = |data: &'static str, errs, res: &'static str, err, dbuflen| {
        let r = BadReader {
            data: data.as_bytes().to_vec(),
            errs,
            ..Default::default()
        };

        TestCase {
            r,
            res: res.to_string(),
            err,
            dbuflen,
        }
    };

    // some test case in golang is discarded due the distinct bahaviors between golang's io.Reader
    // and rust std::io::Read
    let mut test_vector = vec![new_test_case(
        "MY======",
        vec![fake_bad_error()],
        "",
        fake_bad_error(),
        0,
    )];

    for (i, tc) in test_vector.iter_mut().enumerate() {
        let input = String::from_utf8_lossy(tc.r.data.as_slice()).to_string();
        let mut decoder = Decoder::new(STD_ENCODING.clone(), &mut tc.r);
        let dbuflen = if tc.dbuflen > 0 {
            tc.dbuflen
        } else {
            STD_ENCODING.decoded_len(input.len())
        };
        let mut dbuf = vec![0u8; dbuflen];
        let mut res = vec![];
        let mut err = None;
        while err.is_none() {
            match decoder.read(&mut dbuf) {
                Ok(n) => {
                    res.extend_from_slice(&dbuf[..n]);
                    break;
                }
                Err(e) => err = Some(e),
            }
        }

        let got = String::from_utf8_lossy(res.as_slice()).to_string();
        assert_eq!(tc.res, got, "#{} decoding of {}", i, input);
        assert_eq!(tc.err.is_some(), err.is_some());
        if err.is_some() {
            assert_eq!(
                tc.err.as_ref().unwrap().kind(),
                err.unwrap().kind(),
                "#{} unexpected error kind",
                i
            );
        }
    }
}
*/

#[test]
fn new_line_characters() {
    let test_string_encoding = |expected, examples: &[&str]| {
        for (i, v) in examples.iter().enumerate() {
            let buf = STD_ENCODING.decode_string(v).unwrap();
            let got = String::from_utf8_lossy(&buf).to_string();
            assert_eq!(expected, got, "#{} decode({})", i, v);
        }
    };

    let examples = &[
        "ON2XEZI=",
        "ON2XEZI=\r",
        "ON2XEZI=\n",
        "ON2XEZI=\r\n",
        "ON2XEZ\r\nI=",
        "ON2X\rEZ\nI=",
        "ON2X\nEZ\rI=",
        "ON2XEZ\nI=",
        "ON2XEZI\n=",
    ];
    test_string_encoding("sure", examples);

    let examples = &["MZXW6YTBOI======", "MZXW6YTBOI=\r\n====="];
    test_string_encoding("foobar", examples);
}

#[test]
fn reader_eof() {
    let input: &'static str = "MZXW6YTB";
    let br = BadReader {
        data: input.as_bytes().to_vec(),
        errs: vec![None, Some(io::Error::new(ErrorKind::Other, "bad error"))],
        ..Default::default()
    };

    let mut decoder = Decoder::new(STD_ENCODING.clone(), br);
    let mut dbuf = vec![0u8; STD_ENCODING.decoded_len(input.len())];
    let _ = decoder.read(&mut dbuf).unwrap();

    for _ in 0..2 {
        let err = decoder.read(&mut dbuf).unwrap_err();
        assert_eq!(ErrorKind::Other, err.kind(), "unexpected error kind");
        assert!(
            err.to_string().ends_with("bad error"),
            "unexpected error cause"
        );
    }
}

#[test]
fn with_custom_padding() {
    for (i, testcase) in pairs().iter().enumerate() {
        let default_padding = STD_ENCODING.encode_to_string(testcase.decoded.as_bytes());
        let custom_padding = STD_ENCODING
            .clone()
            .with_padding(Some(b'@'))
            .encode_to_string(testcase.decoded.as_bytes());

        let expected = default_padding.replace('=', "@");
        assert_eq!(expected, custom_padding, "#{} custom failed", i);
        assert_eq!(testcase.encoded, default_padding, "#{} standard failed", i)
    }
}

#[test]
fn without_padding() {
    for (i, testcase) in pairs().iter().enumerate() {
        let default_padding = STD_ENCODING.encode_to_string(testcase.decoded.as_bytes());
        let custom_padding = STD_ENCODING
            .clone()
            .with_padding(None)
            .encode_to_string(testcase.decoded.as_bytes());

        let expected = default_padding.trim_end_matches('=');

        assert_eq!(expected, custom_padding, "#{} custom failed", i);
        assert_eq!(testcase.encoded, default_padding, "#{} standard failed", i)
    }
}

#[test]
fn without_padding_close() {
    let encodings = [
        STD_ENCODING.clone(),
        STD_ENCODING.clone().with_padding(None).clone(),
    ];

    for (i, encoding) in encodings.iter().enumerate() {
        for (j, testpair) in pairs().iter().enumerate() {
            let mut buf = vec![];
            let mut encoder = Encoder::new(encoding.clone(), &mut buf);
            encoder.write(testpair.decoded.as_bytes()).unwrap();
            encoder.flush().unwrap();

            let expected = if encoding.pad_char.is_some() {
                testpair.encoded.to_string()
            } else {
                testpair.encoded.replace('=', "")
            };

            let got = String::from_utf8_lossy(&buf).to_string();
            assert_eq!(
                expected,
                got,
                "#{}-#{} with padding({})",
                i,
                j,
                encoding.pad_char.is_some()
            );
        }
    }
}

#[derive(Default)]
struct BadReader {
    data: Vec<u8>,
    errs: Vec<Option<io::Error>>,
    limit: usize,
}

impl Read for BadReader {
    // Populates p with data, returns a count of the bytes written and an
    // error.  The error returned is taken from badReader.errs, with each
    // invocation of Read returning the next error in this slice, or io.EOF,
    // if all errors from the slice have already been returned.  The
    // number of bytes returned is determined by the size of the input buffer
    // the test passes to decoder.Read and will be a multiple of 8, unless
    // badReader.limit is non zero.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.errs.is_empty() {
            if let Some(v) = self.errs.remove(0) {
                return Err(v);
            }
        }

        let lim = {
            let mut ell = buf.len();
            if (self.limit != 0) && (self.limit < ell) {
                ell = self.limit;
            }
            usize::min(self.data.len(), ell)
        };

        builtin::copy(buf, &self.data[..lim]);
        self.data = self.data[lim..].to_vec();

        Ok(lim)
    }
}

struct Testpair<'a> {
    decoded: &'a str,
    encoded: &'a str,
}

impl<'a> Testpair<'a> {
    fn new(decoded: &'a str, encoded: &'a str) -> Self {
        Self { decoded, encoded }
    }
}

fn bigtest() -> Testpair<'static> {
    Testpair::new(
        "Twas brillig, and the slithy toves",
        "KR3WC4ZAMJZGS3DMNFTSYIDBNZSCA5DIMUQHG3DJORUHSIDUN53GK4Y=",
    )
}

fn pairs() -> Vec<Testpair<'static>> {
    vec![
        // RFC 4648 examples
        Testpair::new("", ""),
        Testpair::new("f", "MY======"),
        Testpair::new("fo", "MZXQ===="),
        Testpair::new("foo", "MZXW6==="),
        Testpair::new("foob", "MZXW6YQ="),
        Testpair::new("fooba", "MZXW6YTB"),
        Testpair::new("foobar", "MZXW6YTBOI======"),
        // Wikipedia examples, converted to base32
        Testpair::new("sure.", "ON2XEZJO"),
        Testpair::new("sure", "ON2XEZI="),
        Testpair::new("sur", "ON2XE==="),
        Testpair::new("su", "ON2Q===="),
        Testpair::new("leasure.", "NRSWC43VOJSS4==="),
        Testpair::new("easure.", "MVQXG5LSMUXA===="),
        Testpair::new("asure.", "MFZXK4TFFY======"),
        Testpair::new("sure.", "ON2XEZJO"),
    ]
}

use std::{
    io::{self, ErrorKind, Read, Write},
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
};

use base32::{CorruptInputError, Decoder, STD_ENCODING};

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
fn buffered_decoding_padding() {
    struct Case {
        chunks: Vec<String>,
        expected_err: String,
    }

    fn new<const N: usize>(chunks: [&str; N], expected_err: &str) -> Case {
        Case {
            chunks: chunks.iter().map(|v| v.to_string()).collect(),
            expected_err: expected_err.to_string(),
        }
    }

    let test_vector = vec![
        new(["I4======", "=="], "unexpected end of file"),
        new(
            ["I4======N4======"],
            "illegal base64 data '=' at input byte 2 after writing 0 bytes",
        ),
        // this impl supports decodes well-formatted base32 chunks, differing itself from golang's.
        //new(
        //    ["I4======", "N4======"],
        //    "illegal base64 data '=' at input byte 0 after writing 0 bytes",
        //),
    ];

    for (i, c) in test_vector.iter().enumerate().skip(2) {
        let Case {
            chunks,
            expected_err,
        } = c;
        let (pr, mut pw) = new_io_pipe();

        std::thread::scope(|s| {
            s.spawn(move || {
                for v in chunks {
                    let _ = pw.write(v.as_bytes());
                }
            });
        });

        let mut decoder = Decoder::new(*STD_ENCODING, pr);
        let mut discarded = vec![];

        let got = decoder.read_to_end(&mut discarded).unwrap_err();
        assert!(
            got.to_string().contains(expected_err.as_str()),
            "#{i}: expect '{expected_err}', got '{got}'"
        );
    }
}

#[test]
fn decoder_buffering() {
    let bigtest = &testbot::BIGTEST;
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

        let expect = testbot::escape_ascii_string(bigtest.decoded);
        let got = String::from_utf8_lossy(&buf[..total]).to_string();
        assert_eq!(expect, got, "Decoding/{} of {}", bs, bigtest.encoded);
    }
}

#[test]
fn decoder() {
    for (i, v) in testbot::PAIRS.iter().enumerate() {
        let mut decoder = Decoder::new(STD_ENCODING.clone(), v.encoded.as_bytes());
        let mut dbuf = vec![0u8; STD_ENCODING.decoded_len(v.encoded.len())];
        let count = decoder.read(&mut dbuf).unwrap();
        assert_eq!(v.decoded.len(), count, "#{} Read from {}", i, v.encoded);

        let expect = testbot::escape_ascii_string(v.decoded);
        let got = String::from_utf8_lossy(&dbuf[..count]).to_string();
        assert_eq!(expect, got, "#{} decode of {}", i, v.encoded);

        if count != 0 {
            let count = decoder.read(&mut dbuf).unwrap();
            assert_eq!(0, count, "#{} 2nd read should trigger EOF", i);
        }
    }
}

/// Verifies decode errors are propagated when there are no read errors.
#[test]
fn error() {
    const INPUT: &'static str = "MZXW6YTb";
    let mut dbuf = vec![0u8; STD_ENCODING.decoded_len(INPUT.len())];
    let br = BadReader {
        data: INPUT.as_bytes().to_vec(),
        ..Default::default()
    };

    let mut decoder = Decoder::new(STD_ENCODING.clone(), br);
    let got = decoder
        .read(&mut dbuf)
        .expect_err("got no error")
        .into_inner()
        .expect("miss error src")
        .downcast::<CorruptInputError>()
        .expect("case as CorruptInputError");

    let expect = CorruptInputError {
        c: None,
        idx: 7,
        written: 0,
    };

    assert_eq!(&expect, got.as_ref(), "non-corrupted input error");
}

/// Tests that decoder.Read behaves correctly when the caller supplied reader returns an error.
/// @warning: This is incomplete.
#[test]
fn golang_issue20044() {
    let fake_bad_error = || Some(io::Error::new(ErrorKind::Other, "bad reader error"));

    struct TestCase {
        r: BadReader,
        res: String,
        err: Option<io::Error>,
        dbuflen: usize,
    }

    fn new(
        data: &'static str,
        errs: Vec<Option<io::Error>>,
        res: &'static str,
        err: Option<io::Error>,
        dbuflen: usize,
    ) -> TestCase {
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
    }

    // some test case in golang is discarded due the distinct bahaviors between golang's io.Reader
    // and rust std::io::Read
    let mut test_vector = vec![
        // check valid input data accompanied by an error is processed and the error is propagated.
        new("MY======", vec![fake_bad_error()], "", fake_bad_error(), 0),
        // Check a read error accompanied by input data consisting of newlines only is propagated.
        new(
            "\n\n\n\n\n\n\n\n",
            vec![fake_bad_error()],
            "",
            fake_bad_error(),
            0,
        ),
        // Reader will be called twice.  The first time it will return 8 newline characters.  The
        // second time valid base32 encoded data and an error.  The data should be decoded
        // correctly and the error should be propagated.
        new(
            "\n\n\n\n\n\n\n\nMY======",
            vec![None, fake_bad_error()],
            "",
            fake_bad_error(),
            8,
        ),
    ];

    for (i, tc) in test_vector.iter_mut().enumerate().skip(2) {
        let input = String::from_utf8_lossy(tc.r.data.as_slice()).to_string();
        let mut decoder = Decoder::new(*STD_ENCODING, &mut tc.r);
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

        let got = testbot::escape_ascii_string(res.as_slice());
        assert_eq!(tc.res, got, "#{i} decoding of {input}");

        assert_eq!(tc.err.as_ref().map(|v| v.kind()), err.map(|v| v.kind()));
    }
}

#[test]
fn golang_issue4779() {
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

/// Ensures decoder::read behaves correctly when input data is exhausted.
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
    let _ = decoder.read(&mut dbuf).expect("1st read should be ok");

    let err = decoder
        .read(&mut dbuf)
        .expect_err("2nd read should error out");
    assert_eq!(ErrorKind::Other, err.kind(), "unexpected error kind");
    assert!(
        err.to_string().ends_with("bad error"),
        "unexpected error cause"
    );

    assert_eq!(
        0,
        decoder.read(&mut dbuf).expect("Ok(0)"),
        "3rd read should trigger EOF"
    );
}

#[derive(Default)]
struct BadReader {
    data: Vec<u8>,
    errs: Vec<Option<io::Error>>,
    limit: usize,
}

struct PipeReader {
    buf: Mutex<Vec<u8>>,
    recv: Receiver<Vec<u8>>,
}

struct PipeWriter {
    sender: Sender<Vec<u8>>,
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

impl Read for PipeReader {
    fn read(&mut self, p: &mut [u8]) -> io::Result<usize> {
        let mut buf = self.buf.lock().unwrap();
        if !buf.is_empty() {
            let n = builtin::copy(p, buf.as_slice());
            let _ = buf.drain(..n);
            return Ok(n);
        }

        match self.recv.recv() {
            Ok(mut v) => {
                let n = builtin::copy(p, v.as_slice());
                let _ = v.drain(..n);
                *buf = v;
                Ok(n)
            }
            Err(_) => Ok(0),
        }
    }
}

impl Write for PipeWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.sender.send(buf.to_vec()).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn new_io_pipe() -> (PipeReader, PipeWriter) {
    let (sender, recv) = mpsc::channel();
    let buf = Mutex::default();

    let r = PipeReader { buf, recv };
    let w = PipeWriter { sender };

    (r, w)
}

mod testbot;

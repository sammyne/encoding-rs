use crate::Error;
use std::io;

fn test_constant(value: u64, max_len: usize) {
    let mut buf = vec![0u8; crate::MAX_VARINT_LEN64];
    let n = crate::put_uvarint(buf.as_mut_slice(), value);
    assert_eq!(max_len, n);
}

fn test_overflow(b: &[u8], n0: isize) {
    let (x, n) = crate::uvariant(b);
    assert_eq!(0, x, "uvariant({:?}) expects x=0, got {}", b, x);
    assert_eq!(n0, n, "uvariant({:?}) expects n={}, got {}", b, n0, n);

    let mut b = b;
    match crate::read_uvarint(&mut b) {
        Ok(_) => panic!("read_uvarint() should error out when overflow"),
        Err(Error::Overflow) => {}
        Err(err) => panic!("read_uvarint() returns a unexpected error: {:?}", err),
    }
}

fn test_uvarint(x: u64) {
    let mut buf = vec![0u8; crate::MAX_VARINT_LEN64];
    let n = crate::put_uvarint(buf.as_mut_slice(), x);

    let (y, m) = crate::uvariant(&buf[..n]);

    assert_eq!(x, y, "uvarint({}): got {}", x, y);
    assert_eq!(n, m as usize, "uvarint({}): expect n = {}, got {}", x, n, m);

    let mut reader = buf.as_slice();
    let y = crate::read_uvarint(&mut reader)
        .map_err(|err| format!("read_uvarint({}): {:?}", x, err))
        .expect("fail to read_uvarint");
    assert_eq!(x, y, "read_uvarint({}): got {}", x, y);
}

fn test_varint(x: i64) {
    let mut buf = vec![0u8; crate::MAX_VARINT_LEN64];
    let n = crate::put_varint(buf.as_mut_slice(), x);

    let (y, m) = crate::variant(&buf[..n]);

    assert_eq!(x, y, "varint({}): got {}", x, y);
    assert_eq!(n, m as usize, "varint({}): expect n = {}, got {}", x, n, m);

    let mut reader = buf.as_slice();
    let y = crate::read_varint(&mut reader)
        .map_err(|err| format!("read_varint({}): {:?}", x, err))
        .expect("fail to read_varint");
    assert_eq!(x, y, "read_varint({}): got {}", x, y);
}

fn test_vector() -> Vec<i64> {
    vec![
        -1 << 63,
        (-1 << 63) + 1,
        -1,
        0,
        1,
        2,
        10,
        20,
        63,
        64,
        65,
        127,
        128,
        129,
        255,
        256,
        257,
        (u64::MAX >> 1) as i64,
    ]
}

#[test]
fn buffer_too_small() {
    let buf = [0x80u8; 4];
    for i in 0..=buf.len() {
        let b = &buf[..i];
        let (x, n) = crate::uvariant(b);

        assert_eq!(0, x, "uvariant({:?}): expect x=0, got {}", b, x);
        assert_eq!(0, n, "uvariant({:?}): expect n=0, got {}", b, n);

        let mut r = b;
        match crate::read_uvarint(&mut r) {
            Ok(_) => panic!("should panic"),
            Err(Error::IO(err, n)) => {
                assert_eq!(io::ErrorKind::UnexpectedEof, err.kind());
                assert_eq!(
                    b.len(),
                    n,
                    "expecting {} bytes read, but got {}",
                    b.len(),
                    n
                );
            }
            Err(err) => panic!("unexpected error: {:?}", err),
        }
    }
}

#[test]
fn contants() {
    test_constant(u16::MAX as u64, crate::MAX_VARINT_LEN16);
    test_constant(u32::MAX as u64, crate::MAX_VARINT_LEN32);
    test_constant(u64::MAX as u64, crate::MAX_VARINT_LEN64);
}

#[test]
fn non_canonical_zero() {
    let buf = &[0x80u8, 0x80, 0x80, 0];

    let (x, n) = crate::variant(buf);
    assert_eq!(0, x);
    assert_eq!(4, n);
}

#[test]
fn overflow() {
    test_overflow(
        &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x2],
        -10,
    );
    test_overflow(
        &[
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01, 0, 0,
        ],
        -13,
    );
}

#[test]
fn uvarint() {
    let test_vector = test_vector();

    for x in test_vector {
        test_uvarint(x as u64);
    }

    {
        let mut x = 0x07u64;
        while x != 0 {
            test_uvarint(x);
            x <<= 1;
        }
    }
}

#[test]
fn varint() {
    let test_vector = test_vector();
    for x in test_vector {
        test_varint(x);
        test_varint(x.overflowing_neg().0);
    }
}

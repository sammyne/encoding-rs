use crate::binary;
use crate::Error;
use std::io;

fn test_constant(value: u64, max_len: usize) {
    let mut buf = vec![0u8; binary::MAX_VARINT_LEN64];
    let n = binary::put_uvarint(buf.as_mut_slice(), value);
    assert_eq!(max_len, n);
}

fn test_overflow(b: &[u8], n0: isize) {
    let (x, n) = binary::uvariant(b);
    assert_eq!(0, x, "uvariant({:?}) expects x=0, got {}", b, x);
    assert_eq!(n0, n, "uvariant({:?}) expects n={}, got {}", b, n0, n);

    match binary::read_uvarint(b) {
        Ok(_) => panic!("read_uvarint() should error out when overflow"),
        Err(Error::Overflow) => {}
        Err(err) => panic!("read_uvarint() returns a unexpected error: {:?}", err),
    }
}

fn test_uvarint(x: u64) {
    let mut buf = vec![0u8; binary::MAX_VARINT_LEN64];
    let n = binary::put_uvarint(buf.as_mut_slice(), x);

    let (y, m) = binary::uvariant(&buf[..n]);

    assert_eq!(x, y, "uvarint({}): got {}", x, y);
    assert_eq!(n, m as usize, "uvarint({}): expect n = {}, got {}", x, n, m);

    let y = binary::read_uvarint(buf.as_slice())
        .map_err(|err| format!("read_uvarint({}): {:?}", x, err))
        .expect("fail to read_uvarint");
    assert_eq!(x, y, "read_uvarint({}): got {}", x, y);
}

fn test_varint(x: i64) {
    let mut buf = vec![0u8; binary::MAX_VARINT_LEN64];
    let n = binary::put_varint(buf.as_mut_slice(), x);

    let (y, m) = binary::variant(&buf[..n]);

    assert_eq!(x, y, "varint({}): got {}", x, y);
    assert_eq!(n, m as usize, "varint({}): expect n = {}, got {}", x, n, m);

    let y = binary::read_varint(buf.as_slice())
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
        let (x, n) = binary::uvariant(b);

        assert_eq!(0, x, "uvariant({:?}): expect x=0, got {}", b, x);
        assert_eq!(0, n, "uvariant({:?}): expect n=0, got {}", b, n);

        match binary::read_uvarint(b) {
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
    test_constant(u16::MAX as u64, binary::MAX_VARINT_LEN16);
    test_constant(u32::MAX as u64, binary::MAX_VARINT_LEN32);
    test_constant(u64::MAX as u64, binary::MAX_VARINT_LEN64);
}

#[test]
fn non_canonical_zero() {
    let buf = &[0x80u8, 0x80, 0x80, 0];

    let (x, n) = binary::variant(buf);
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

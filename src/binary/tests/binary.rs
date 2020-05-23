use std::panic;

use crate::binary::{ByteOrder, LittleEndian};

#[test]
fn uint64_small_slice_length_panics() {
    let panicked = panic::catch_unwind(|| {
        let b = [1u8, 2, 3, 4, 5, 6, 7, 8];
        LittleEndian::uint64(&b[..4]);
    })
    .is_err();

    assert!(panicked, "missing panic");
}

#[test]
fn put_uint64_small_slice_length_panics() {
    let panicked = panic::catch_unwind(|| {
        let mut b = [0u8; 8];
        LittleEndian::put_uint64(&mut b[..4], 0x0102030405060708);
    })
    .is_err();

    assert!(panicked, "missing panic");
}

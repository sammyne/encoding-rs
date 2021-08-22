use encoding::base32::STD_ENCODING;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn decode(c: &mut Criterion) {
    const N: usize = 8192;

    let data = {
        let mut data = vec![0u8; STD_ENCODING.encoded_len(N)];
        STD_ENCODING.encode(&mut data, &[0u8; N]);
        data
    };

    let mut buf = vec![0u8; N];
    c.bench_function("decode", |b| {
        b.iter(|| {
            let _ = STD_ENCODING.decode(&mut buf, black_box(&data));
        })
    });
}

fn decode_string(c: &mut Criterion) {
    let data = STD_ENCODING.encode_to_string(&[0u8; 8192]);
    let mut out = vec![];
    c.bench_function("decode_string", |b| {
        b.iter(|| {
            out = STD_ENCODING.decode_string(black_box(&data)).unwrap();
        })
    });
}

fn encode(c: &mut Criterion) {
    let data = [0u8; 8192];

    let mut buf = vec![0u8; STD_ENCODING.encoded_len(data.len())];
    c.bench_function("encode", |b| {
        b.iter(|| {
            STD_ENCODING.encode(&mut buf, black_box(&data));
        })
    });
}

fn encode_to_string(c: &mut Criterion) {
    let data = [0u8; 8192];

    let mut out = String::new();
    c.bench_function("encode_to_string", |b| {
        b.iter(|| {
            out = STD_ENCODING.encode_to_string(black_box(&data));
        })
    });
}

criterion_group!(benches, decode, decode_string, encode, encode_to_string);
criterion_main!(benches);

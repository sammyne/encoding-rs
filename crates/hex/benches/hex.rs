use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn dump(c: &mut Criterion) {
    let sizes = &[256usize, 1024, 4096, 16384];

    for size in sizes {
        let src = [2u8, 3, 5, 7, 9, 11, 13, 17].repeat(size / 8);
        let mut out = String::new();

        let size_string = format!("{}", size);
        c.bench_function(&size_string, |b| {
            b.iter(|| {
                out = hex::dump(black_box(src.as_slice()));
            })
        });
    }
}

criterion_group!(benches, dump);
criterion_main!(benches);

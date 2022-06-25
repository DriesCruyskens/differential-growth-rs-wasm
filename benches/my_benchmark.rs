use criterion::{black_box, criterion_group, criterion_main, Criterion};
use differential_growth;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Run Differential Line", |b| {
        b.iter(|| {
            let mut line = differential_growth::Line::new(
                black_box(100.0),
                black_box(100.0),
                black_box(10),
                black_box(200.0),
                black_box(1.5),
                black_box(1.0),
                black_box(9.0),
                black_box(0.9),
                black_box(5.0),
            );
            for _ in 0..200 {
                line.run();
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_differential_growth;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Run Differential Line", |b| {
        b.iter(|| {
            let mut line = rust_differential_growth::RustDifferentialGrowth::new(
                black_box(100.0),
                black_box(100.0),
                black_box(10),
                black_box(10.0),
                black_box(1.5),
                black_box(1.0),
                black_box(14.0),
                black_box(1.1),
                black_box(5.0),
                black_box(200.0),
                black_box(200.0)
            );
            for _ in 0..500 {
                line.tick();
            }
        })
    });
}

// Decreasing sample size since the default takes way to long with high ticks.
// High number of ticks are preferred so we test the system with a high number of nodes.
// https://bheisler.github.io/criterion.rs/book/user_guide/advanced_configuration.html#configuring-sample-count--other-statistical-settings
criterion_group! {
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10).measurement_time(Duration::from_secs(30));
    targets = criterion_benchmark
}
criterion_main!(benches);

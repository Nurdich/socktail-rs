use criterion::{criterion_group, criterion_main, Criterion};

fn throughput_benchmark(c: &mut Criterion) {
    c.bench_function("placeholder", |b| {
        b.iter(|| {
            // Placeholder benchmark
            1 + 1
        })
    });
}

criterion_group!(benches, throughput_benchmark);
criterion_main!(benches);

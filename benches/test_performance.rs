use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_factorize::factorize;

pub fn test_performance(c: &mut Criterion) {
    c.bench_function("fact 10_000", |b| {
        b.iter(|| factorize::prime_factors(black_box(10_000)))
    });
}

criterion_group!(benches, test_performance);
criterion_main!(benches);

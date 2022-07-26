use criterion::{black_box, Criterion, criterion_group, criterion_main};

use rust_factorize::factorize;

pub fn test_performance(c: &mut Criterion) {
    c.bench_function("fact 37", |b| {
        b.iter(|| factorize::prime_factors(black_box(37)))
    });
    c.bench_function("fact 10_000", |b| {
        b.iter(|| factorize::prime_factors(black_box(10_000)))
    });
    c.bench_function("fact 1_000_000_000", |b| {
        b.iter(|| factorize::prime_factors(black_box(1_000_000_000)))
    });
    c.bench_function("fact (2^48) - 3", |b| {
        b.iter(|| factorize::prime_factors(black_box((2_u64.pow(48)) - 3)))
    });
}

criterion_group!(benches, test_performance);
criterion_main!(benches);


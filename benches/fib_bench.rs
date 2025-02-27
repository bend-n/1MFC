use criterion::{Criterion, criterion_group, criterion_main};
use onebfc::*;

fn fib_bench(c: &mut Criterion) {
    c.bench_function("fib 1_000_000", |b| b.iter(|| code::fib(1000000)));
}

criterion_group!(benches, fib_bench);
criterion_main!(benches);

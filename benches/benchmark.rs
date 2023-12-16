use criterion::{criterion_group, criterion_main, Criterion};

use mylib::fibo;

fn criterion_benchmark(c: &mut Criterion) {
    for i in 0..10 {
        c.bench_function(&format!("fibo_{}", i), |b| b.iter(|| fibo(i)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

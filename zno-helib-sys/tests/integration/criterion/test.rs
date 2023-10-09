use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark(c: &mut Criterion) {
    c.bench_function("hello_world", |b| b.iter(|| 2 + 2));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

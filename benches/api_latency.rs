use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_auth(c: &mut Criterion) {
    c.bench_function("auth_check", |b| b.iter(|| black_box(1 + 1)));
}
criterion_group!(benches, bench_auth);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quicksort_rs::quicksort;

fn quicksort_benchmark(c: &mut Criterion) {
    let data = vec![33, 10, 55, 71, 29, 3, 18, 44, 90, 2, 1, 0, 99, 1000];

    c.bench_function("quicksort", |b| {
        b.iter(|| quicksort(&mut black_box(data.clone())))
    });
}

criterion_group!(benches, quicksort_benchmark);
criterion_main!(benches);

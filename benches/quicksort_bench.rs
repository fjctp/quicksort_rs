// Import necessary Criterion macros and types
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Import your quicksort function from the crate
use quicksort_rs::quicksort;

/// Benchmark function for the in-place quicksort algorithm.
///
/// This function creates a sample dataset, clones it for each iteration,
/// and passes it to quicksort. Cloning ensures that each iteration
/// gets a fresh unsorted input.
///
/// `black_box` prevents the compiler from optimizing away data usage,
/// ensuring that the benchmark remains accurate.
fn quicksort_benchmark(c: &mut Criterion) {
    let data = vec![33, 10, 55, 71, 29, 3, 18, 44, 90, 2, 1, 0, 99, 1000];

    c.bench_function("quicksort", |b| {
        b.iter(|| quicksort(&mut black_box(data.clone())))
        // - `data.clone()` gives a fresh unsorted Vec for each run
        // - `black_box(...)` prevents compiler from optimizing away the call
        // - `&mut [...]` is passed because quicksort sorts in-place
    });
}

// Register the benchmark group to be run by Criterion
criterion_group!(benches, quicksort_benchmark);

// Define the main entry point for Criterion benchmarks
criterion_main!(benches);

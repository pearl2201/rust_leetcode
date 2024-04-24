use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use rust_leetcode::sorting;
use std::fs;
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn generate_array_with_length(n: usize) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 1..n {
        arr.push(rng.gen_range(0..1000000));
    }
    return arr;
}
fn criterion_benchmark(c: &mut Criterion) {
    let test_sizes: Vec<usize> = vec![10000];

    for test_size in test_sizes {
        c.bench_function(format!("merge_sort_size_{}", test_size).as_str(), |b| {
            b.iter(|| {
                let mut arr = generate_array_with_length(test_size);
                let n = arr.len();
                sorting::merger_sort(black_box(n), black_box(&mut arr));
            })
        });

        // c.bench_function(format!("bubble_sort_size_{}", test_size).as_str(), |b| {
        //     b.iter(|| sorting::bubble_sort(black_box(n), black_box(&mut arr)))
        // });

        // c.bench_function(format!("selection_sort_size_{}", test_size).as_str(), |b| {
        //     b.iter(|| sorting::linear_sort(black_box(n), black_box(arr.clone())))
        // });

        // c.bench_function(format!("insertion_sort_size_{}", test_size).as_str(), |b| {
        //     b.iter(|| sorting::insertion_sort(black_box(n), black_box(arr.clone())))
        // });

        c.bench_function(format!("quick_sort_size_{}", test_size).as_str(), |b| {
            b.iter(|| {
                let mut arr = generate_array_with_length(test_size);
                let n = arr.len();
                sorting::quick_sort_entry(black_box(n), black_box(&mut arr));
            })
        });
    }
}

criterion_group!(name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark);
criterion_main!(benches);

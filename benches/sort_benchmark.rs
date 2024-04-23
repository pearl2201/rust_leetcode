use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_leetcode::sorting;
use std::fs;
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("./assets/bubble_sort.txt").unwrap();
    let arr: Vec<i32> = input
        .split_ascii_whitespace()
        .map(|x| {
            return x.parse::<i32>().unwrap();
        })
        .collect();
    let n = arr.len();

    c.bench_function("bubble_sort", |b| {
        b.iter(|| sorting::bubble_sort(black_box(n), black_box(arr.clone())))
    });

    c.bench_function("linear_sort", |b| {
        b.iter(|| sorting::linear_sort(black_box(n), black_box(arr.clone())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

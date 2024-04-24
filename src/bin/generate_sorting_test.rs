use rand::prelude::*;
use rust_leetcode::sorting::bubble_sort;
use std::fs;
fn main() {
    let mut arr: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 1..10000 {
        arr.push(rng.gen_range(0..1000000));
    }
    let content = format!(
        "{}",
        arr.clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let _ = fs::write("./assets/input_sorting.txt", content);

    let n = arr.len();
    let sorted_arr = bubble_sort(n, &mut arr);

    let content = format!(
        "{}",
        sorted_arr
            .clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let _ = fs::write("./assets/output_sorting.txt", content);
}

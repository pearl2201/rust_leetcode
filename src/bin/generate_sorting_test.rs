use std::fs;
use rand::prelude::*;
fn main()  {
    let mut arr: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 1..10000 {
        arr.push(rng.gen_range(0..100000));
    }
    let content = format!("{}",arr.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","));
    let _ = fs::write("./assets/input_sorting.txt", content);
}

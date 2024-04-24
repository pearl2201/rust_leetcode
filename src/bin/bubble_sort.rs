use std::fs;
fn main() {
    let input = fs::read_to_string("./assets/bubble_sort.txt").unwrap();
    let mut arr: Vec<u32> = input
        .split_ascii_whitespace()
        .map(|x| {
            return x.parse::<u32>().unwrap();
        })
        .collect();

    let mut temp: u32;
    let n = arr.len();
    for i in 0..(n) {
        for j in ((i + 1)..(n)).rev() {
            if arr[j] < arr[j - 1] {
                temp = arr[j - 1];
                arr[j - 1] = arr[j];
                arr[j] = temp;
            }
        }
    }
    println!(
        "{}",
        arr.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

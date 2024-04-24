use rand::Rng;

pub fn print_arr(arr: Vec<i32>) {
    println!(
        "{}",
        arr.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

pub fn generate_arr(length: usize) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        arr.push(rng.gen_range(0..1000000));
    }
    return arr;
}

pub fn print_arr(arr: Vec<i32>) {
    println!(
        "{}",
        arr.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

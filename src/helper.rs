pub fn printArr(arr: Vec<u32>) {
    println!(
        "{}",
        arr.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}

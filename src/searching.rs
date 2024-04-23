use std::fs;

pub fn binary_search() {
    let input = fs::read_to_string("./assets/binary_search.txt").unwrap();
    let lines = input.split('\n').collect::<Vec<_>>();
    let arr: Vec<u32> = lines[0]
        .split_ascii_whitespace()
        .map(|x| {
            return x.parse::<u32>().unwrap();
        })
        .collect();
    println!("{}", lines[1]);
    let m = lines[1].parse::<u32>().unwrap();

    let mut i = 0;
    let mut j = arr.len() - 1;
    let mut found = false;
    let mut k = 0;
    while i <= j && !found {
        let middle_idx = (j + i) / 2;
        let middle = arr[middle_idx];
        if middle == m {
            k = middle_idx;
            found = true;
        } else if middle > m {
            j = middle_idx - 1;
        } else {
            i = middle_idx + 1;
        }
    }
    if found {
        println!("{}", k);
    } else {
        println!("not found");
    }
}

pub fn linear_search() {
    let input = fs::read_to_string("./assets/linear_search.txt").unwrap();
    let lines = input.split('\n').collect::<Vec<_>>();
    let arr: Vec<u32> = lines[0]
        .split_ascii_whitespace()
        .map(|x| {
            return x.parse::<u32>().unwrap();
        })
        .collect();
    println!("{}", lines[1]);
    let m = lines[1].parse::<u32>().unwrap();

    let n = arr.len();
    let mut k = 0;
    let mut found: bool = false;
    for i in 0..(n) {
        if arr[i] == m {
            k = i;
            found = true;
            break;
        }
    }
    if found {
        println!("{}", k);
    } else {
        println!("not found");
    }
}

use std::{fmt::Debug, fs, mem::swap};

use crate::helper;
pub fn bubble_sort(n: usize, arr: &mut Vec<i32>) -> Vec<i32> {
    let mut temp: i32;
    for i in 0..(n) {
        for j in ((i + 1)..(n)).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
    return arr.to_vec();
}

pub fn linear_sort(n: usize, mut arr: Vec<i32>) -> Vec<i32> {
    let mut temp: i32;
    for i in 0..n {
        let mut arg_temp: usize = i;
        temp = arr[i];
        for j in i + 1..n {
            if arr[j] < temp {
                temp = arr[j];
                arg_temp = j;
            }
        }
        if arg_temp != i {
            arr[arg_temp] = arr[i];
            arr[i] = temp;
        }
    }
    return arr;
}

pub fn insertion_sort(n: usize, mut arr: Vec<i32>) -> Vec<i32> {
    for i in 1..n {
        let curr = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > curr {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = curr;
    }
    return arr;
}

pub fn merger_sort(n: usize, arr: &mut Vec<i32>) -> Vec<i32> {
    //println!("{:?}", arr);
    merger_sort_partial(arr, 0, n - 1);
    return arr.to_vec();
}

pub fn merger_sort_partial(arr: &mut Vec<i32>, l: usize, u: usize) {
    if l < u {
        let k: usize = (u - l) / 2 + l;
        merger_sort_partial(arr, l, k);
        merger_sort_partial(arr, k + 1, u);
        merge(arr, l, k, u);
    }
}

pub fn quick_sort_entry(n: usize, arr: &mut Vec<i32>) -> Vec<i32> {
    quick_sort(arr, 0, n - 1);
    return arr.to_vec();
}

pub fn quick_sort(arr: &mut Vec<i32>, l: usize, u: usize) {
    if l < u {
        let p = quick_sort_partition(arr, l, u);
        //println!("{} - {} - {}", l, p, u);
        quick_sort(arr, l, p);
        quick_sort(arr, p + 1, u);
    }
}

pub fn quick_sort_partition(arr: &mut Vec<i32>, l: usize, u: usize) -> usize {
    let pivot = arr[(u + l) / 2];
    let mut i = l;
    let mut j = u;
    // println!(
    //     "-----------\nsorting: {:?},\n pivot: {}, lower: {}, upper: {}",
    //     arr, pivot, l, u
    // );
    loop {
        while arr[i] < pivot {
            i += 1;
        }

        while arr[j] > pivot {
            j -= 1;
        }

        if i >= j {
            break;
        }

        arr.swap(i, j);
        i += 1;
        j -= 1;
    }
    // println!("sorting {:?},result: {}", arr, j);
    return j;
}

pub fn merge(arr: &mut Vec<i32>, l: usize, m: usize, u: usize) {
    //println!("---\n Merge: {:?}, lower: {}, middle: {}, upper: {}", arr, l, m, u);
    //helper::print_arr(arr.clone());
    let mut i = l;
    let mut j = m + 1;
    let mut temp: Vec<i32> = Vec::new();
    while i <= m && j <= u {
        if arr[i] < arr[j] {
            temp.push(arr[i]);
            i += 1;
        } else {
            temp.push(arr[j]);
            j += 1;
        }
    }
    //println!("---\n Fill i: {:?}, {:?}", arr,temp);
    while i <= m {
        temp.push(arr[i]);
        i += 1;
    }
    //println!("---\n Fill j: {:?}, {:?}", arr,temp);
    while j <= u {
        temp.push(arr[j]);
        j += 1;
    }
    //println!("---\n Copy: {:?}, {:?}", arr,temp);
    for k in l..(u + 1) {
        arr[k] = temp[k - l];
    }
    //helper::print_arr(arr.clone());
}

#[test]
fn test_linear_sort() {
    let arr: Vec<i32> = load_input();
    let n: usize = arr.len();
    let sorted_arr = linear_sort(n, arr);
    assert!(check_output(sorted_arr));
}

#[test]
fn test_bubble_sort() {
    let mut arr: Vec<i32> = load_input();
    let n: usize = arr.len();
    let sorted_arr = bubble_sort(n, &mut arr);
    assert!(check_output(sorted_arr));
}

#[test]
fn test_insertion_sort() {
    let arr: Vec<i32> = load_input();
    let n: usize = arr.len();
    let sorted_arr = insertion_sort(n, arr);
    assert!(check_output(sorted_arr));
}

#[test]
fn test_merger_sort() {
    let mut arr: Vec<i32> = load_input();
    let n: usize = arr.len();
    let sorted_arr = merger_sort(n, &mut arr);
    assert!(check_output(sorted_arr.to_vec()));
}

#[test]
fn test_quick_sort() {
    let mut arr: Vec<i32> = load_input();
    let n: usize = arr.len();
    let sorted_arr = quick_sort_entry(n, &mut arr);
    assert!(check_output(sorted_arr.to_vec()));
}

#[test]
fn test_quick_sort_simple() {
    let mut arr: Vec<i32> = vec![3, 2, 1, 5, 4];
    let n: usize = arr.len();
    let sorted_arr = quick_sort_entry(n, &mut arr);
    helper::print_arr(sorted_arr.clone());
    assert!(check_arr_equal(sorted_arr.to_vec(), vec![1, 2, 3, 4, 5]));
}

#[test]
fn test_quick_sort_100() {
    let mut arr: Vec<i32> = helper::generate_arr(100);

    let n: usize = arr.len();
    let output = bubble_sort(n, &mut arr);
    let sorted_arr = quick_sort_entry(n, &mut arr);
    helper::print_arr(sorted_arr.clone());
    assert!(check_arr_equal(sorted_arr.to_vec(), output));
}

#[test]
fn test_merger_sort_simple() {
    let mut arr: Vec<i32> = vec![3, 2, 1, 5, 4];
    let n: usize = arr.len();
    let sorted_arr = merger_sort(n, &mut arr);
    helper::print_arr(sorted_arr.clone());
    assert!(check_arr_equal(sorted_arr.to_vec(), vec![1, 2, 3, 4, 5]));
}

fn load_input() -> Vec<i32> {
    let input = fs::read_to_string("./assets/input_sorting.txt").unwrap();
    let arr: Vec<i32> = input
        .split(',')
        .map(|x| {
            return x.parse::<i32>().unwrap();
        })
        .collect();
    return arr;
}

fn load_output() -> Vec<i32> {
    let input = fs::read_to_string("./assets/output_sorting.txt").unwrap();
    let arr: Vec<i32> = input
        .split(',')
        .map(|x| {
            return x.parse::<i32>().unwrap();
        })
        .collect();
    return arr;
}

fn check_output(arr: Vec<i32>) -> bool {
    let output = load_output();
    for i in 0..arr.len() {
        if arr[i] != output[i] {
            return false;
        }
    }

    return true;
}

pub fn check_arr_equal(arr: Vec<i32>, output: Vec<i32>) -> bool {
    for i in 0..arr.len() {
        if arr[i] != output[i] {
            return false;
        }
    }

    return true;
}

pub fn quicksort<T: Eq + PartialEq + Clone + PartialOrd + Debug>(
    arr: &mut Vec<T>,
    low: usize,
    high: usize,
) {
    println!("{:?}", arr);
    if low < high {
        let p = partition(arr, low, high, &|a, b| a <= b);
        quicksort(arr, low, p - 1);
        quicksort(arr, p + 1, high);
    }
}
fn partition<T: Clone, F: Fn(&T, &T) -> bool>(
    arr: &mut Vec<T>,
    low: usize,
    high: usize,
    f: &F,
) -> usize {
    let pivot = match arr.get(high) {
        Some(v) => v.clone(),
        _ => {
            panic!("Array index {:?} out of bounds", high)
        }
    };
    let mut i = low;
    for j in low..high - 1 {
        match arr.to_vec().get(j) {
            Some(v) => {
                if f(v, &pivot) {
                    &arr.swap(i, j);
                    i += 1;
                }
            }
            _ => {
                panic!("Array index {:?} for j out of bounds", j)
            }
        }
    }
    &arr.swap(i, high);
    i
}

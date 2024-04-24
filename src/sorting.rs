use std::{fs, mem::swap};

use crate::helper;
pub fn bubble_sort(n: usize, mut arr: Vec<i32>) -> Vec<i32> {
    let mut temp: i32;
    for i in 0..(n) {
        for j in ((i + 1)..(n)).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
    return arr;
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

#[test]
fn test_linear_sort() {
    let arr: Vec<i32> = load_input();
    let n: usize = arr.len();
    let sorted_arr = linear_sort(n, arr);
    assert!(check_output(sorted_arr));
}

#[test]
fn test_bubble_sort() {
    let arr: Vec<i32> = load_input();
    let n: usize = arr.len();
    let sorted_arr = bubble_sort(n, arr);
    assert!(check_output(sorted_arr));
}

#[test]
fn test_insertion_sort() {
    let arr: Vec<i32> = load_input();
    let n: usize = arr.len();
    let sorted_arr = insertion_sort(n, arr);
    assert!(check_output(sorted_arr));
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

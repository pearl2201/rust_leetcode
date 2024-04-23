pub fn bubble_sort(n: usize, mut arr: Vec<i32>) -> Vec<i32> {
    let mut temp: i32;
    for i in 0..(n) {
        for j in ((i + 1)..(n)).rev() {
            if arr[j] < arr[j - 1] {
                temp = arr[j - 1];
                arr[j - 1] = arr[j];
                arr[j] = temp;
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

#[test]
fn test_linear_sort() {
    let n: usize = 5;
    let arr: Vec<i32> = vec![2, 4, 5, 1, 3];
    let sorted_arr = linear_sort(n, arr);
    let n2 = sorted_arr[2];
    let n0 = sorted_arr[0];
    assert_eq!(n2, 3);
    assert_eq!(n0, 1);
    assert_eq!(sorted_arr[4], 5);
}

#[test]
fn test_bubble_sort() {
    let n: usize = 5;
    let arr: Vec<i32> = vec![2, 4, 5, 1, 3];
    let sorted_arr = bubble_sort(n, arr);
    let n2 = sorted_arr[2];
    let n0 = sorted_arr[0];
    assert_eq!(n2, 3);
    assert_eq!(n0, 1);
    assert_eq!(sorted_arr[4], 5);
}


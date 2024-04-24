use std::time::SystemTime;

use rust_leetcode::{
    helper,
    sorting::{self, bubble_sort, check_arr_equal},
};

fn main() {
    let start_time = SystemTime::now();

    let mut arr: Vec<i32> = helper::generate_arr(1000000);

    let n: usize = arr.len();
    let sorted_arr = sorting::merger_sort(n, &mut arr);
    match start_time.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
}

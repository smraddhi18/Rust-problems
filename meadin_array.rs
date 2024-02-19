use std::io;

fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (arr[mid_left] as f64 + arr[mid_right] as f64) / 2.0
    } else {
        arr[len / 2] as f64
    }
}

fn main() {
    println!("Enter a sorted array of integers separated by spaces:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let median_value = median(&numbers);
    println!("The median of the array is: {}", median_value);
}

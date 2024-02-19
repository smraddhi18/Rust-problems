use std::io;

fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort(); 

    Some(sorted_arr[k - 1]) 
}

fn main() {
    println!("Enter a list of integers separated by spaces:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    println!("Enter the value of k:");
    let mut k_input = String::new();
    io::stdin()
        .read_line(&mut k_input)
        .expect("Failed to read line");

    let k: usize = k_input.trim().parse().expect("Invalid value of k");

    match kth_smallest_element(&numbers, k) {
        Some(element) => println!("The {}th smallest element is: {}", k, element),
        None => println!("Invalid value of k or array too short."),
    }
}

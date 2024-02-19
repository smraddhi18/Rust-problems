use std::io;

fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            // Check if this is the first occurrence of the target
            if mid == 0 || arr[mid - 1] < target {
                return Some(mid);
            } else {
                high = mid - 1; // Move to the left to find the first occurrence
            }
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
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

    println!("Enter the number to search for:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let target: i32 = input.trim().parse().expect("Invalid number");

    if let Some(index) = first_occurrence_index(&numbers, target) {
        println!("The first occurrence of {} is at index {}.", target, index);
    } else {
        println!("{} is not found in the array.", target);
    }
}

use std::io;

fn main() {
    println!("Enter a string to reverse:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

   
    input = input.trim().to_string();

   
    let reversed_string: String = input.chars().rev().collect();

    println!("Reversed string: {}", reversed_string);
}

use std::io;

fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase(); 
    let reversed_input: String = input.chars().rev().collect(); 

    input == reversed_input 
}

fn main() {
    println!("Enter a string to check if it's a palindrome:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); 

    if is_palindrome(input) {
        println!("\"{}\" is a palindrome.", input);
    } else {
        println!("\"{}\" is not a palindrome.", input);
    }
}

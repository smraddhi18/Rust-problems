use std::io;

fn shortest_word(input: &str) -> Option<&str> {
    input.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    println!("Enter a string of words:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    match shortest_word(&input) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the string"),
    }
}

use std::io;

fn main() {
    let mut input = String::new();

    // Get the string of integers

    println!("Please enter a list of space-separated integers.");

    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read input");

    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    // Iterate over the vector in reverse order and print

    for num in numbers.iter().rev() {
        print!("{num} ");
    }
}

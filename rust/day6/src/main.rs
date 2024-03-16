use std::io;
use std::io::Write;

fn main() {
    // Get the number of strings
    let mut input = String::new();
    let mut num_strings = 0;

    let mut strings: Vec<String> = Vec::new();

    println!("Please enter the number of strings");

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read integer");

        num_strings = match input.trim().parse::<i32>() {
            Ok(num_strings) => num_strings,
            Err(_) => {
                println!("Please enter an integer!");
                input = String::new(); // empty the variable of input
                continue;
            },
        };
        break;
    }

    // Get the strings
    for i in 0..num_strings {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read string");

        strings.push(input);
    }

    // Divide the odd and even characters for each string
    for s in strings {
        let mut evens: Vec<char> = Vec::new();
        let mut odds: Vec<char> = Vec::new();

        for c in s.chars() {
            if &evens.len() < &odds.len() {
                evens.push(c);
            }
            else {
                odds.push(c);
            }
        }
        // print the results
        let length: usize = odds.len();
        println!("{length}");
        for c in &odds {
            print!("{c}");
        }
        print!(" ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        for c in &evens {
            print!("{c}");
        }
        println!("");
    }
}

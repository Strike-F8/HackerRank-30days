use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    let mut phonebook = HashMap::new();
    let mut num_entries = 0;

    // get the number of entries in the phonebook
    println!("Please enter the number of entires in the phonebook.");

    loop
    {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read integer");

        num_entries = match input.trim().parse::<i32>() {
            Ok(num_entries) => num_entries,
            Err(_) => {
                println!("Please enter an integer!");
                input = String::new(); // empty the variable of input
                continue;
            },
        };
        break;
    }


    // get the phonebook entries

    for i in 0..num_entries {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read string");
        let mut entry = input.split_whitespace();
        let mut key = entry.remainder();
        entry.next();
        let mut value = entry.remainder();
        phonebook.insert(key, value);
    }
}

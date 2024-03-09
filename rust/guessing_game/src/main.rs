use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number from 1 to 10!");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    
    let mut num_guesses: u32 = 0;

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };

        println!("You guessed: {guess}");
        num_guesses += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("You guessed {num_guesses} times.");
}

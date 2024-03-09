use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    let mut num = 0;
    
    println!("Enter an integer from 1 to 100 and I will determine its weirdness.");
    
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read integer");

        num = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer from 1 to 100!");
                input = String::new(); // empty the variable of old input
                continue;
            },
        };
        break;
    }

    println!("You entered: {num}");
    if num%2 == 1
    {
        println!("Weird");
    }
    else
    {
        match num.cmp(&5) {
            Ordering::Less => println!("Not Weird"),
            Ordering::Equal => println!("Not Weird"),
            Ordering::Greater => {
                match num.cmp(&20) {
                    Ordering::Less => println!("Weird"),
                    Ordering::Equal => println!("Weird"),
                    Ordering::Greater => println!("Not Weird")
                }
            }
        }
    }
}

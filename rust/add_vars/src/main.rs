use std::io;

fn main() {
    // Declare static variables
    let i: i32 = 4;
    let d: f32 = 4.0;
    let mut s: String = "Knock knock ".to_string();

    // Declare variables for input
    let mut input = String::new();
    let mut num: i32 = 0;
    let mut dub: f32 = 0.0;
    let mut sentence = String::new();

    // Retrieve input from stdin
    println!("Enter an integer");
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read integer");

        num = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer!");
                input = String::new(); //empty the variable of old input
                continue;
            },
        };
        break;
    }

    println!("Enter a decimal");
    input = String::new(); //empty the variable of old input
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read decimal");

        dub = match input.trim().parse::<f32>() {
            Ok(dub) => dub,
            Err(_) => {
                println!("Please enter a decimal number!");
                input = String::new(); //empty the variable of old input
                continue;
            },
        };
        break;
    }

    println!("Enter a sentence");
    input = String::new(); //empty the variable of old input
    loop {
        io::stdin()
            .read_line(&mut sentence)
            .expect("Failed to read sentence");

        sentence = match sentence.trim().parse() {
            Ok(sentence) => sentence,
            Err(_) => {
                println!("Please enter a sentence!");
                input = String::new(); //empty the variable of old input
                continue;
            },
        };
        break;
    }

    // Add constants to input

    num = num + i;
    dub = dub + d;
    s.push_str(&sentence);

    // Print results

    println!("{num}");
    println!("{dub}");
    println!("{s}");
}

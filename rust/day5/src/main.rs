use std::io;

fn main() {
    let mut input = String::new();
    let mut num = 0;

    println!("Enter an integer and I will calculate its first 10 multiples.");

    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read integer");

        num = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer!");
                input = String::new(); // empty the variable of old input
                continue;
            },
        };
        break;
    }

    for i in 1..11 {
        let product: i32 = num*i;
        println!("{num} x {i} = {product}");
    }
}

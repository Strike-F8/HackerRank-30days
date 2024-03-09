use std::io;

fn main() {
    let mut input = String::new();
    let mut meal_cost: f64 = 0.0;
    let mut tip_percent: i32 = 0;
    let mut tax_percent: i32 = 0;
    // Read meal cost
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read meal cost");

        meal_cost = match input.trim().parse() {
            Ok(meal_cost) => meal_cost,
            Err(_) => {
                println!("Please enter the meal cost!");
                continue;
            },
        };
        break;
    }

    // Read tax percent
    loop {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read tax percent");

        tax_percent = match input.trim().parse() {
            Ok(tax_percent) => tax_percent,
            Err(_) => {
                println!("Please enter the tax percent as an integer!");
                continue;
            },
        };
        break;
    }

    // Read tax percent
    loop {
        input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read tax percent");

        tip_percent = match input.trim().parse() {
            Ok(tip_percent) => tip_percent,
            Err(_) => {
                println!("Please enter the tip percent as an integer!");
                continue;
            },
        };
        break;
    }
    let result: f64 = math::round::half_up(calc_total(meal_cost, tax_percent, tip_percent),0);
    println!("{result}");
}

fn calc_total(meal_cost: f64, tax_percent: i32, tip_percent: i32) -> f64 {
    let tax: f64 = meal_cost * ((tax_percent as f64)/100.0);
    let tip: f64 = meal_cost * ((tip_percent as f64)/100.0);

    meal_cost + tax + tip
}

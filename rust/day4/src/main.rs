#![allow(non_snake_case)]
use std::cmp::Ordering;

struct Person {
    age: i32
}

impl Person {
    fn new(initialAge: i32) -> Person {
        if initialAge < 0
        {
            println!("Age is not valid, setting age to 0.");
            return Person { age: 0 };
        }
        
        return Person { age: initialAge };
        
    }
 
    fn amIOld(&self) {
       match self.age.cmp(&13) {
            Ordering::Less => println!("You are young."),
            Ordering::Equal => println!("You are a teenager."),
            Ordering::Greater => {
                match self.age.cmp(&18) {
                    Ordering::Less => println!("You are a teenager."),
                    Ordering::Equal => println!("You are old."),
                    Ordering::Greater => println!("You are old."),
                }
            }
       }
        
    }
 
    fn yearPasses(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let T: i32 = read_line().trim().parse().unwrap();
 
    for _ in 0..T {
        let age = read_line().trim().parse().unwrap();
        let mut p = Person::new(age);
 
        p.amIOld();
 
        for _ in 0..3 {
            p.yearPasses();
        }
 
        p.amIOld();
        println!("");
    }
}

 fn read_line() -> String {
   let mut  input = String::new();
   std::io::stdin().read_line(&mut input).expect("Could not read stdin!");
    return input;
    

}

/*
   Compare numbers
   Develop a program that accepts user inputs, compares it with a randomly generated number
*/
use colored::*;
use rand::Rng;
use std::{
    cmp::{self, Ordering},
    io,
};


fn main() {
    let secret_number = rand::thread_rng().gen_range(0..1000);

    loop {
        println!("{}", "Enter a number".cyan());
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("please enter a number");

        let input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you entered: {}", &input);

        match input.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "Equal, you won".blue());
                break;
            }
            Ordering::Greater => println!("{}", "Your number is greater".red()),
            Ordering::Less => println!("{}", "Your number is smaller".red()),
        }
    }
}

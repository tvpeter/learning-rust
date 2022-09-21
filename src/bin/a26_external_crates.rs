/*
Topic: External Crates

Requirements:
- display the current date and time

Notes:
- Use the 'chrono' crate to work with time
- (Optional) Read the documentation section 'formatting and parsing' for the examples on how to create 
custom time formats
*/
use chrono::prelude::*;

fn main(){
    let current_date = Local::now();

    println!("Current date: {}, current time: {}", current_date.date(), current_date.time());

    println!("Formatted date: {}, formatted time: {}", current_date.date().format("%d-%b-%Y").to_string(), current_date.time().format("%H:%M:%S:%P").to_string());

}

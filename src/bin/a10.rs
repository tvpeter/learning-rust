/**
 * ACTIVITY 10
 * Expressions
 * print it's big if a variable is >100
 * print it's small if a variable is <100
 * 
 * NOTE
 * use a boolean variable to set to an expression that determines if a variable is >100 or <100
 * use a fn to print the messages
 * use a match expression to determine which message to print
 */

 fn main(){
    let count = 101;

    let is_big_value = count > 100;

    print_message(is_big_value);
    
 }

 fn print_message(is_big_value: bool) {
    match is_big_value {
        true => println!("its big"),
        false => println!("its small"),
    }
 }

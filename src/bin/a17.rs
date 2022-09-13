/**
 * topic: browsing standard library documentation
 * 
 * Requirements:
 * print a string in lowercase and uppercase
 * 
 * Notes:
 * utilize standard library functionality to transform the string to lowercase and uppercase
 * use `rustup doc` in a terminal to open the standard library
 * Navigate to the API documentation section
 * search for functionality to transform a string (or str) to uppercase and lowercase
 * try searching for: to_uppercase, to_lowercase
 */

fn main () {
    let text = "This is a string";

    println!("In lowercase: {:?}", text.to_lowercase());
    println!("In uppercase: {:?}", text.to_uppercase());
}

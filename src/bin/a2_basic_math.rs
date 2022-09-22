/*
display the result of the sum of two numbers
-use a fn to add two numbers
- use a fn to display the result
use the {:?} in the println to display the result
*/

fn main () {
    let result = add_numbers(-13, 200);
    display_sum(result);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn display_sum (number: i32) {
    println!("{:?}", number);
}

use core::num;

/**
 * Topic: vectors
 * 
 * requirements
 * print 10, 20, "thirty" and 40 in a loop
 * print the total number of elements in a vector
 * 
 * Notes
 * Use a vector to store 4 numbers
 * iterate through the vector using a for ..in loop
 * determine whether to print the number or print "thirty" inside
 * use the .len() function to print the number of elements
 */


 fn main() {
    let numbers = vec![10, 20, 30, 40];

    for num in &numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{}", num),
        };
    }

    println!("number of elements: {}", &numbers.len());
 }

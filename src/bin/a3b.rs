/**
 * display ">5", "<5", or "=5" based on the value of a variable
 * nOTES
 * Use a variable to set the value for the integer
 * use if else if block to determine which message to display
 * use the println macro
 **/

fn main () {

    let given_num = 7;

    if given_num > 5 {
        println!(">5");
    }
    else if given_num < 5 {
        println!("<5");
    }
    else {
        println!("=5");
    }
}





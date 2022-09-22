/*
Display a message based on the boolean value of a variable
- if the variable is true, display 'hello'
- if the variable is false, display 'goodbye'

-use a variable set to either true or false
- use an if --else block
use the println macro
*/

fn main() {

    let status: bool = false;

    if !status {
        println!("goodbye");
    }
    else {
        println!("hello")
    }

}



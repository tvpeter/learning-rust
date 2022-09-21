/*
Topic: External modules

Summary:
The existing program is complete, but all the code exists in a single module
This code can benefit from been organised into multiple external modules

Requirements:
Organise the code into two external modules based on their functionality
- msg: string formatting functions
- math: math functions
update the main function to use the functionality from the module

Notes:

- update your cargo.toml to include a library file
- after moving the functions into modules, try running 'cargo check --bin a26_external_modules' to get
a list of required code change
 */

fn main() {
    use activity::math;

    let result = {
        let two_plus_two = math::add(2, 2);
        let three = math::sub(two_plus_two, 1);
        math::mul(three, three)
    };

    //ensure we have correct result

    assert_eq!(result, 9);
    println!("(2 + 2 -1) * 3 = {}", result);

    //part 2 - string functions

    {
        use activity::msg::{capitalize, exciting, trim};

        let hello = {
            let msg = "hello ";
            let msg = trim(msg);
            capitalize(msg)
        };

        let world = {
            let msg = "world";
            exciting(msg)
        };

        let msg = format!("{}, {}", hello, world);

        //ensure we have the correct result
        assert_eq!(&msg, "Hello, world!");
        println!("{}", msg);
    }
}

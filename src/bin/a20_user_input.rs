/*
    Topic: User input

    Requirements:
    - verify user input against pre-defined keywords
    - the keywords represent possible power options for a computer:
        - off
        - sleep
        - Reboot
        - Shutdown
        - Hibernate
    - if the user enters one of the keywords, a message should be printed to the console indicating which action will be taken
     - example: if the user types in 'shutdown' a message should display such as 'shutting down'
     - if the keyword entered does not exist, an appropriate error message should be displayed

     Notes:
     - use an enum to store the possible power states
     - use a function with a match expression to print out the power messages
      - the function should accept the enum as an input
    - use a match expression to convert the user input into the power state enum
    - the program should be case-insensitive (the user should be able to type, Reboot, reboot, REBOOT, etc)
*/
use std::io;

enum PowerState {
    OFF,
    SLEEP,
    REBOOT,
    SHUTDOWN,
    HIBERNATE,
}

impl PowerState {
    fn new(user_input: &str) -> Option<PowerState> {
        let user_input = user_input.trim().to_lowercase();

        match user_input.as_str() {
            "off" => Some(PowerState::OFF),
            "sleep" => Some(PowerState::SLEEP),
            "rebooth" => Some(PowerState::REBOOT),
            "shutdown" => Some(PowerState::SHUTDOWN),
            "hibernate" => Some(PowerState::HIBERNATE),
            _ => None
        }
    }

}

fn print_action(state: PowerState) {
        match state {
        PowerState::OFF => println!("System going off"),
        PowerState::SLEEP => println!("System going to sleep"),
        PowerState::REBOOT => println!("System reboothing"),
        PowerState::SHUTDOWN => println!("System shutting down"),
        PowerState::HIBERNATE => println!("System hibernating"),
    }
}

fn main() {
    let mut buffer = String::new();
    println!("Enter a power state: ");
    let input = io::stdin().read_line(&mut buffer);

    if input.is_ok() {
        match PowerState::new(&buffer) {
            Some(state) => print_action(state),
            None => println!("Invalid power state"),
        }
    } else {
        println!("error reading input");
    }
}

/**
 * Topic: Result
 *
 * Requirements:
 * determine if a customer is able to make a restricted purchase
 * restricted purchases require that the age of the customer is at least 21
 *
 * Notes:
 * Use a struct to store at least the age of a customer
 * use a function to determine if a customer can make a restricted purchase
 * Return a result from the function
 * The Err variant should detail the reason why they cannot make a purchase
 */

struct Customer {
    age: i32,
}

fn purchase_eligibility(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        Ok(())
    }else {
        Err("Customer must be atleast 21 years to be eligible".to_owned())
    }
}

fn main() {

    let nkem = Customer { age: 20 };
    
    let check = purchase_eligibility(&nkem);

    match check {
        Ok(()) => println!("Customer can make purchase"),
        Err(error) => println!("Error: {:?}", error),
    }

}

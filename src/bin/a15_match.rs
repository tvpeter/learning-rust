/**
 * Topic: Advanced match
 * 
 * Requirements: 
 * print out a list of tickets and their information for an event
 * tickets can be Backstage, Vip and Standard
 * Backstage and Vip tickets include the ticket holder's name
 * all tickets include the price
 * 
 * Notes
 * Use an enum for the tickets with data associated with each variant
 * create one of each ticket and place into a vector
 * use a match expression while iterating the vector to print the ticket info
 */

 enum Ticket {
    Backstage(f64, String),
    ViP(f64, String),
    Standard(f64),
 }


 fn main() {

    let ticket_holders = vec![
        Ticket::Backstage(14.50, "Jonas".to_owned()),
        Ticket::ViP(15.9, "Peter".to_owned()),
        Ticket::Standard(10.01),
    ];

    for ticket_holder in ticket_holders {
        match ticket_holder {
            Ticket::Backstage(price, holder) => println!("Backstage ticket: Holder : {}, price: {}", price, holder),
            Ticket::Standard(price) => println!("Standard ticket: price {}", price),
            Ticket::ViP(price, holder) => println!("VIP Ticket, holder {}, and the price is: {}", price, holder),
        }
    }
}

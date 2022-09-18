/**
 * Topic: HashMap
 * 
 * Requirements:
 * print the name and number of items in stock for a furniture store
 * if the number of items is 0, print "out of stock" instead of 0
 * the store has:
 * 5 chairs
 * 3 beds
 * 2 tables
 * 0 couches
 * print the total number of items in stock
 * 
 * notes:
 * use a HashMap for the furniture store stock
 */

 use std::collections::HashMap;

fn main(){

    let mut stock = HashMap::new();
    stock.insert("chairs", 5);
    stock.insert("beds", 3);
    stock.insert("tables", 2);
    stock.insert("couches", 0);

    let mut total_items = 0;
    for (item, qty) in stock.iter() {
        if qty < &1 {
            println!("item: {}, quantity: out of stock", item);
        } else {
        println!("Item: {}, quantity in stock: {}", item, qty);
        }
        total_items += qty;
    }

    println!("Total Stock: {}", total_items);
}

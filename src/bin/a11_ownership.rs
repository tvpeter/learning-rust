/**
 * Topic: Ownership
 * print out the quantity and id number of a grocery item
 * 
 * NOTES:
 * use a struct for the grocery item
 * use two i32 field for the quantity and id number
 * create a fn to display the quantity
 * create a fn to display the id number
 */

 struct Grocery {
    quantity: i32,
    id: i32
 }

 fn print_grocery_qty(grocery_item: &Grocery){
    println!("The quantity: {}", grocery_item.quantity);
 }

 fn print_grocery_id(grocery_item: &Grocery){
    println!("The grocery id: {}", grocery_item.id);
 }
 fn main() {

    let noodles = Grocery {
        quantity: 10,
        id: 1000123,
    };

    print_grocery_id(&noodles);
    print_grocery_qty(&noodles);
 }

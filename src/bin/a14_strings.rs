/**
 * Topic: Strings
 * Requirements:
 * print out the name and favorite colors of people aged 10 and under
 * 
 * NOTES:
 * use a struct for the persons age, name, and favorite color
 * the color and name should be stored as a String
 * create and store at least 3 people in a vector
 * iterate through the vector using a for..in loop
 * use an if expression to determine which person's info should be printed
 * the name and colors should be printed using a function
 */

 struct Person {
    age: i32,
    name: String,
    fav_color: String,
 }

 fn print(person: &Person){
    println!("Name: {}", person.name);
    println!("fav color: {}", person.fav_color);
 }

 fn main() {

    let persons = vec![
        Person { age: 23, name: "Peter Tyonum".to_owned(), fav_color: String::from("Black") },
        Person { age: 2, name: "Anthonia Tyonum".to_owned(), fav_color: String::from("Pink") },
        Person { age: 9, name: "Jane Doe".to_owned(), fav_color: String::from("Blue") },
    ];

    for person in persons {
        if person.age <= 10 {
            print(&person);
        }
    }

 }

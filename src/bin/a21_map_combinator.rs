/**
 * Topic: Map combinator
 * 
 * Requirements:
 * Given a user name, create and print out a User struct if the user exists
 * 
 * Notes:
 * use the existing find_user function to locate a user
 * use the map fn to create the User
 * print out the User struct if found, or a "not found message" if not
 */

 #[derive(Debug)]
 struct User {
    user_id: i32,
    name: String,
 }


 //locate a user id based on the name
 fn find_user(name: &str)->Option<i32> {
    let name = name.to_lowercase();

    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
 }
fn main(){

    let user_name = "Sam";
    let user = find_user(user_name).map(|user_id| User { name: user_name.to_string(), user_id });

    match user {
        Some(user) => println!("user found: {:?}", user),
        None => println!("User not found"),
    }

}

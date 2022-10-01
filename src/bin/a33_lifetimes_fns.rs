/*
Topic: Lifetimes and functions

Summary:
Create a function that compares which string is longer (highest length)

Requirements:
- the comparison must be done using a function named 'longest'
- no data may be copied (cannot use .to_owned() or .to_string())
- if both strings are the same length, the first one should be returned
*/

fn longest<'a>(first_word: &'a str, second_word: &'a str) -> &'a str {
    if second_word.len() > first_word.len() {
        second_word
    }else {
        first_word
    }
}

fn main(){
    let first_word = "Peter";
    let second_word = "Tyonum";

    let longest_word = longest(first_word, second_word);

    println!("{}", longest_word);
}

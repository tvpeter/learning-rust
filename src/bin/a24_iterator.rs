/*
Topic: Iterator

Requirements:
- Triple the value of each item in a vector
- Filter the data to only include values > 10
- print out each element using a for loop

Notes:
Use an iterator chain to accomplish the task
*/

fn main(){

    let numbers = vec![1, 9, 31, 12, 23, 40, 24];

    let new_numbers: Vec<_> = numbers
        .iter()
        .map(|num| num * 3)
        .filter(|num| num > &10)
        .collect();

        for num in new_numbers {
            println!("{}", num);
        }
}

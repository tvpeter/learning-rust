/**
 * Topic: Option
 * 
 * Requirements:
 * print out the details of a students locker assignment
 * lockers use numbers and are optional for students
 * 
 * Notes:
 * use a struct containing the students name and locker assignment
 * the locker assignment should use an Option<i32>
 */

 struct Student {
    name: String,
    locker: Option<i32>
 }

fn main(){
    let students = vec![
        Student { name: "John".to_owned(), locker: Some(12)},
        Student { name: "Terna".to_owned(), locker: Some(1)},
        Student { name: "Peter".to_owned(), locker: None},
    ];

    for student in students {
        println!("Student name: {}", student.name);

        match student.locker {
            Some(num) => println!("Locker {} assigned", num),
            None => println!("Has no locker assigned"),
        }
    }
}

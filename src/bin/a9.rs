//tuple

fn coordinate () -> (i32, i32) {
    (1, 6)
}

fn main() {
    let (x, y) = coordinate();

    if y > 5 {
        println!("y is greater than 5");
    }
    else if y == 5 {
        println!("y is equal to 5");
    }
    else {
        println!("y is less than 5");
    }

    //expression

let my_num = x > 5;

println!("{}", my_num);
}



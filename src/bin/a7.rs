enum Colors {
    Blue,
    Black,
    Green,
    Red,
}

fn main() {
 let color = Colors::Black;

 print_color(color);
 
}

fn print_color(color: Colors) {
    match color {
        Colors::Black => println!("its a black color"),
        Colors::Blue => println!("its a blue color"),
        Colors::Green => println!("its a green color"),
        Colors::Red => println!("its a red color"),
    }
}



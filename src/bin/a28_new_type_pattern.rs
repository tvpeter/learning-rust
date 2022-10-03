/*
Topic: New Type Pattern

Requirements:
- Display the selected color of shoes, a shirt and pants
- create and display at least one of each type of clothes and color

Notes:
- Create a new type for the colors of the clothes
    - each new type should implement a 'new' function
- create a function for each type of clothes that accepts a new type of color specific to that type of clothing
 */

 #[derive(Debug)]
 enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow
 }

 #[derive(Debug)]
 struct ShoesColor(Color);
 impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
 }

 #[derive(Debug)]
 struct ShirtColor(Color);
 impl ShirtColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
 }

 #[derive(Debug)]
 struct PantsColor(Color);
 impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
 }

 fn print_shirt_color(color: ShirtColor) {
    println!("shirt color: {:?}", color);
 }

  fn print_shoes_color(color: ShoesColor) {
    println!("shoes color: {:?}", color);
 }

  fn print_pants_color(color: PantsColor) {
    println!("pants color: {:?}", color);
 }

fn main(){
    let black_shirt = ShirtColor::new(Color::Black);
    print_shirt_color(black_shirt);

    let blue_pants = PantsColor::new(Color::Blue);
    print_pants_color(blue_pants);

    let brow_shoes = ShoesColor::new(Color::Brown);
    print_shoes_color(brow_shoes);
}

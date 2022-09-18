/**
 * Topic: implementing functionality with the impl keyword
 * 
 * requirements:
 * - print the characteristics of a shipping box
 * - must include dimensions, weight and color
 * 
 * notes:
 * use a struct to encapsulate the box characteristics
 * use an enum for the box color
 * implement functionality on the box struct to create a new box
 * implement functionality on the box struct to print the characteristics
 */

 
 enum BoxColor {
    Red,
    Blue,
    Black, 
    Gray,
 }

 impl BoxColor {
     fn print(&self) {
        match self {
            BoxColor::Red => println!("red color"),
            BoxColor::Blue => println!("blue color"),
            BoxColor::Black => println!("black color"),
            BoxColor::Gray => println!("gray color"),
        }
     }
 }
 struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
 }

 impl Dimensions {
     fn print(&self){
        println!("Width: {:?}", self.width);
        println!("depth: {:?}", self.depth);
        println!("Height: {:?}", self.height);
     }
 }

 struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor
 }

 impl ShippingBox {
    fn new(weight: f64, color: BoxColor, dimensions: Dimensions) -> Self {
        Self { 
            weight,
            color,
            dimensions,
        }
    }

    fn print(&self){
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }

     
 }

fn main(){

    let box_dimensions = Dimensions {
        width: 12.0,
        height: 20.0,
        depth: 9.0
    };

    let shipping_box = ShippingBox::new(12.0, BoxColor::Black, box_dimensions);

    shipping_box.print();
}

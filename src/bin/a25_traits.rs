/*
Topic: Traits
Requirements:
-  calculate the perimeter of a square and triangle
    -   The perimeter of a square of the length of any side * 4;
    -   the perimeter of a triange is a+b+c where each variable represent a side
- print out the perimeter of the shapes

NOTES:
-   use a trait to declare the perimeter calculation function
-   use a single function to print out the perimeter of the shapes
-   The fn must utilize impl trait as a function parameter
 */

 trait Shapes {
        fn perimeter(&self) -> f64;
 }

 struct Square {
    side: f64,
 }

 impl Shapes for Square {
    fn perimeter(&self) -> f64 {
        self.side * 4.0
    }
 }

 struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
 }

 impl Shapes for Triangle {
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }
 }

 fn print_shapes(shape: impl Shapes) {
    println!("The perimeter of the given shape is: {}", shape.perimeter());
 }

fn main(){
    let triangle = Triangle{ side_a: 12.0, side_b: 23.1, side_c: 31.2 };
    let sqr = Square{ side: 4.0 };

    print_shapes(triangle);
    print_shapes(sqr);
}

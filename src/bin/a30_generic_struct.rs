/*
    Topic: Generics and Structures
    Requirements:
    - create a Vehicle structure that is generic over traits Body and Color
    - create structures for vehicle bodies and vehicle colors and implement the 
    Body and Color traits for these structures
    - implement a 'new' function for Vehicle that allows it to have any body and any color
    - create at least two different vehicles in the main function and print their info

    notes:
    - examples of car bodies can be Truck, Car, Scooter
    - Examples of colors could be red, white, black
    - it is not necessary to have data fields or function implementations for the vehicle bodies/colors
*/

trait Body {
}

trait Color {
}

#[derive(Debug)]
struct Vehicle<B: Body, C: Color> {
    body: B,
    color: C,
}

impl<B: Body, C: Color> Vehicle<B, C> {
    pub fn new(body: B, color: C) -> Self {
        Self {body, color}
    }
}

#[derive(Debug)]
struct Car;
impl Body for Car{}

#[derive(Debug)]
struct Truck;
impl Body for Truck {}

#[derive(Debug)]
struct Red;

impl Color for Red {}

#[derive(Debug)]
struct Blue;
impl Color for Blue {}

fn main() {

    let truct = Vehicle::new(Truck, Red);
    let car = Vehicle::new(Car, Blue);

    println!("{:?}", truct);
    println!("{:?}", car);

}

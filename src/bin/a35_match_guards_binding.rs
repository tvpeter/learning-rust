/*
    Topic: Match guards and Binding

    Summary:
    - A tile-based game requires different logic for different kinds of tiles.
    print different messages depending on the kind of tile selected

    Requirements:
    - Bricks:
        - Colored bricks should print "The brick color is [color]"
        - Other bricks should print "[Bricktype] brick"
    - Water:
        - pressure levels 10 and over should print "High water pressure"
        - pressure levels under 10 should print "water pressure level: [pressure]"
    - Grass, Dirt, and Sand should all print "Ground tile"
    - Treasure Chests:
        - if the treasure is Gold and the amount is at least 100, print "Lots of Gold"
    - Everything else should not print any messages

    Notes:
    - use a single match expression utilizing guards to implement the program
    - Run the program and print the messages with at least 4 different tiles
*/

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}
#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}
fn main() {
    let brick = Tile::Water(Pressure(9));

    match brick {
        Tile::Brick(b @ BrickStyle::Gray | b @ BrickStyle::Red) => {
            println!("The brick color is {:?}", b)
        }
        Tile::Brick(other) => println!("{:?} brick", other),
        Tile::Water(Pressure(10..)) => println!("High water pressure"),
        Tile::Water(l @ Pressure(0..=9)) => println!("Water pressure level: {:?}", l.0),
        // Tile::Water(pressure) if pressure.0 < 10 => println!("Water pressure level: {:?}", pressure.0),
        Tile::Treasure(TreasureChest {
            amount,
            content:TreasureItem::Gold,
        }) if amount >= 100 => println!("Lots of Gold"),
        Tile::Dirt | Tile::Grass  | Tile::Sand => println!("Ground tile"),
        other => println!("other brick, {:?}", other),
    }
}

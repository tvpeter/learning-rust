enum Flavour {
    Soft,
    Bitter,
    Sweet,
}

struct Drink {
    flavour: Flavour,
    flavour_oz: u32,
}

fn display_drinks(drink: Drink) {
    match drink.flavour {
        Flavour::Bitter => println!("bitter flavour"),
        Flavour::Soft => println!("soft flavour"),
        Flavour::Sweet => println!("sweet flavour"),
    }

    println!("oz: {}", drink.flavour_oz);
}
fn main() {
    let fanta = Drink {
        flavour: Flavour::Soft,
        flavour_oz: 30
    };

    display_drinks(fanta);
}

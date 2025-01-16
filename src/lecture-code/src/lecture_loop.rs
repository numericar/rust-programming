pub fn process() {
    let treasures: [&str; 4] = ["gold", "silver", "ruby gem", "emerald"];

    let mut enegy: u8 = 5;

    for treature in treasures.iter() {
        if enegy == 0 {
            println!("Out of enegy");
            break;
        } else if treature == &"ruby gem" {
            println!("You found!");
        } else {
            enegy -= 1;
            continue;
        }
    }
}
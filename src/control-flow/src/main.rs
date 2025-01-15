fn main() {
    if_statement();
    match_statement();
    loop_statement();
}

fn if_statement() {
    let weather: &str = "rainy";

    if weather == "sunny" {
        println!("Crabby will cross the river by swimming!");
    } else if weather == "rainy" {
        println!("Crabby will build a bride to stay dry");
    } else {
        println!("Crabby will want weather better");
    }
}

fn match_statement() {
    let monster: &str = "dragon";

    match monster {
        "goblin" => println!("Sword to attack"),
        "troll" => println!("sets a trap"),
        "dragon" => println!("run"),
        _ => println!("Confused"),
    }
}

fn loop_statement() {
    let mut wood_count: i16 = 0;

    loop {
        wood_count += 1;

        println!("Current wood: {}", wood_count);

        if wood_count == 10 {
            break;
        }
    }

    println!("Crabby finished the boot");
}
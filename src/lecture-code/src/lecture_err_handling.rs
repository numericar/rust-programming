pub fn process() {
    let chest_result_optinal: Option<String> = open_chest(false);
    let chest_result: bool;
    if let Some(result) = chest_result_optinal {
        println!("{}", result);
        chest_result = true;
    } else {
        println!("{}", "The chest is empty");
        chest_result = false;
    }
    
    let door_result: String = match open_door(false) {
        Ok(safe) => safe,
        Err(mimic) => panic!("{}", mimic)
    };

    println!("chest_result: {}", chest_result);
    println!("door_result: {}", door_result);
}

fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        return None;
    } else {
        return Some("You found a item".to_string());
    }
}

fn open_door(is_danger: bool) -> Result<String, String> {
    if is_danger {
        return Err("You found a monster".to_string());
    } else {
        return Ok("The door is safe".to_string());
    }
}
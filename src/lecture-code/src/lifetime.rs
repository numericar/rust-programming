pub fn process() {
    let map1 = "Bangkok";
    let map2 = "Colorado";

    let chosen_map = longest_map(map1, map2);   
    println!("Crabby's longest map: {}", chosen_map);
}

fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        return map1;
    } else {
        return map2;
    }
}
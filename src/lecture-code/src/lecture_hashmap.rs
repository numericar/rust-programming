use std::collections::HashMap;

pub fn process() {
    let mut fruits: HashMap<&str, i8> = HashMap::new();

    fruits.insert("apple", 10);
    fruits.insert("banana", 5);

    if let Some(apple) = fruits.get_mut("apple") {
        *apple += 5;
    }

    println!("{:?}", fruits);
}
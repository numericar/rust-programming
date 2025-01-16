pub fn process() {
    let mut fruits: Vec<String> = Vec::new();

    fruits.push("apple".to_string());
    fruits.push("banana".to_string());
    fruits.push("orange".to_string());

    fruits.remove(2);

    println!("Fruits: {:?}", fruits);
}
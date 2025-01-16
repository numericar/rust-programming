pub fn process() {
    let numbers: Vec<i32> = vec![100, 200, 300, 400];
    let results: Vec<i32> = numbers.iter().map(|x| x * 2).collect();

    println!("{:?}", results);
}
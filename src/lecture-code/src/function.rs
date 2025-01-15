pub fn process() {
    let result: String = crabby_tasks("gathering coins", 12);
    println!("{}", result);

    let result_2: String = crabby_tasks("cooking", 30);
    println!("{}", result_2);

    let result_3: String = crabby_tasks("hunting", 60);
    println!("{}", result_3);
}

fn crabby_tasks(task: &str, time: i32) -> String {
    return format!("Crabby has successfully done {} in {} minute!", task, time);
}
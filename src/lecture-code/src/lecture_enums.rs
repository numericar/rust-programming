enum CrabbyState {
    Fighting,
    Collecting(u32),
    Defending
}

impl CrabbyState {
    fn state_represent(&self) {
        match self {
            CrabbyState::Fighting => println!("Crabby is fightinh"),
            CrabbyState::Collecting(amount) => println!("Crabby is collecting {}", amount),
            CrabbyState::Defending => println!("Crabby is defending")
        }
    }
}

pub fn process() {
    let fighting = CrabbyState::Fighting;
    let collecting = CrabbyState::Collecting(15);
    let defending = CrabbyState::Defending;

    fighting.state_represent();
    collecting.state_represent();
    defending.state_represent();
}
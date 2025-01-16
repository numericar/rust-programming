struct Inventory<T> {
    item: T
}

trait DisplayItem {
    fn display(&self);
}

impl<T> DisplayItem for Inventory<T> where T: std::fmt::Debug {
    fn display(&self) {
        println!("{:?}", self.item);
    }
}

pub fn process() {
    let gold = Inventory {
        item: 100
    };
    gold.display();

    let sword = Inventory {
        item: "KITIRO"
    };
    sword.display();
}
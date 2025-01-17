use std::{cell::RefCell, rc::Rc};

pub fn process() {
    let chest: Box<i32> = Box::new(10);

    let shared_chest = Rc::new(RefCell::new(chest));

    **shared_chest.borrow_mut() += 10;
    **shared_chest.borrow_mut() += 5;

    println!("Gold: {}", shared_chest.borrow());
}
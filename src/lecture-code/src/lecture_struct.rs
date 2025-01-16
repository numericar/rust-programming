struct Crabby {
    name: String,
    health: u8
}

impl Crabby {
    fn take_damage(&mut self, damage: u8) {
        // saturating จะช่วยป้องกันไม่ให้เกิดการ overflow ของข้อมูล เช่น 
        // u8 จะเก็บข้อมูลเลขได้ตั้งแต่ 0 ~ 128 ถ้าหากมีการใส่ 300 เข้ามาจะเกิดข้อผิดะลาด
        // saturating จะทำให้ข้อมูลที่ใส่เข้ามาคำนวนไม่มากและน้อยกว่าค่าที่สามารถเก็ได้
        self.health = self.health.saturating_sub(damage);
    }

    fn healing(&mut self, heal: u8) {
        if self.health + heal >= 100 {
            self.health = 100;
        } else {
            self.health += heal;
        }
    }
}

pub fn process() {
    let mut crabby: Crabby = Crabby {
        name: "Crabby".to_string(),
        health: 100
    };

    crabby.take_damage(10);
    println!("Crabby health: {}", crabby.health);

    crabby.healing(20);
    println!("Crabby health: {}", crabby.health);
}
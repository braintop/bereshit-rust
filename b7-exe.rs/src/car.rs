use crate::person::Person;

pub struct Car {
    pub brand: String,
    pub model: String,
    pub owner: Person,
}

impl Car {
    pub fn new(brand: String, model: String, owner: Person) -> Self {
        Car { brand, model, owner }
    }//

    pub fn change_owner(&mut self, new_owner: Person) {
        self.owner = new_owner;
    }

    pub fn print(&self) {
        println!("Car: {} {}", self.brand, self.model);
        println!("Owner:");
        self.owner.print();
    }
}

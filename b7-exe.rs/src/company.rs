use crate::address::Address;
use crate::person::Person;

pub struct Company {
    pub name: String,
    pub address: Address,
    pub ceo: Person,
}

impl Company {
    pub fn new(name: String, address: Address, ceo: Person) -> Self {
        Company { name, address, ceo }
    }

    pub fn promote_ceo(&mut self) {
        self.ceo.age += 1;
    }

    pub fn print(&self) {
        println!("Company: {}", self.name);
        self.address.print();
        println!("CEO:");
        self.ceo.print();
    }
}

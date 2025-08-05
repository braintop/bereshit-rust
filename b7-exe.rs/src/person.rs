use crate::address::Address;


#[derive(Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub address: Address,
}

impl Person {
    pub fn new(name: String, age: u8, address: Address) -> Self {
        Person { name, age, address }
    }

    pub fn print(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
        self.address.print();
    }
}

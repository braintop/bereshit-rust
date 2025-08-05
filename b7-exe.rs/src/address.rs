#[derive(Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip: u32,
}

impl Address {
    pub fn new(street: String, city: String, zip: u32) -> Self {
        Address { street, city, zip }
    }

    pub fn print(&self) {
        println!("Address: {}, {}, {}", self.street, self.city, self.zip);
    }
}

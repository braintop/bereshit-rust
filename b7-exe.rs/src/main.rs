mod address;
mod person;
mod company;
mod car;

use address::Address;
use person::Person;
use company::Company;
use car::Car;

fn main() {
    let address = Address::new("Herzl St".to_string(), "Tel Aviv".to_string(), 12345);
    let person = Person::new("Alice".to_string(), 30, address.clone());
    let mut company = Company::new("TechCorp".to_string(), address.clone(), person.clone());

    company.print();
    company.promote_ceo();
    println!("After promotion:");
    company.print();

    let mut car = Car::new("Toyota".to_string(), "Corolla".to_string(), person.clone());
    car.print();

    let new_owner = Person::new("Bob".to_string(), 40, address);
    car.change_owner(new_owner);
    println!("After changing owner:");
    car.print();
}

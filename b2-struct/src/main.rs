// src/main.rs

mod user; // ייבוא הקובץ user.rs

use user::User;

fn main() {   
    let u = User {
        username: String::from("alice"),
        age: 30,
    };

    println!("User: {}, age: {}", u.username, u.age);
}

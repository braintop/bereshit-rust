// src/user.rs

pub struct User {
    pub username: String,
    pub age: i32,
    pub active: bool,
}

impl User {
    /// בנאי שמחזיר משתמש חדש עם שם ברירת מחדל
    pub fn new() -> Self {
        User {
            username: String::from("oren"), // תיקון חשוב: צריך String, לא &str
            age: 0,
            active: true,
        }
    }

    pub fn print(&self) {
        println!("Username: {}, Age: {}, Active: {}", self.username, self.age, self.active);
    }

    pub fn birthday(&mut self) {
        self.age += 1;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

// src/main.rs

mod user;           // טוען את המודול user.rs
use user::User;     // משתמש ב־User מהמודול

fn main() {
    let mut u = User::new();
    u.print();
    u.birthday();
    u.deactivate();
    u.print();
}

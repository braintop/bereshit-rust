// src/main.rs

mod user;
mod worker;

use worker::Worker;

fn main() {
    let mut worker = Worker::new("Alice".to_string(), "Engineer".to_string());
    worker.print();
    worker.user.birthday(); // גישה לפונקציה של User מתוך Worker
    worker.print();
}

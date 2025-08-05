mod user;
mod worker;

use user::User;
use worker::Worker;

fn main() {
    let mut u = User::new("Alice".to_string());
    u.birthday();    // מעלה את הגיל
    u.deactivate();  // מכבה את המשתמש – כעת הפונקציה בשימוש ✅

    let w = Worker::new(u, "Engineer".to_string());
    w.print();
}

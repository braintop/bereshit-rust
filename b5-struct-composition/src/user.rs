
pub struct User {
    pub username: String,
    pub age: i32,
    active: bool
}

impl User {
    /// בנאי שמקבל שם משתמש ומחזיר משתמש חדש
    pub fn new(username: String) -> Self {
        User {
            username,
            age: 0,
            active: true,
        }
    }

    /// פונקציה שמדפיסה את המשתמש
    pub fn print(&self) {
        println!("Username: {}, Age: {}, Active: {}", self.username, self.age, self.active);
    }

    /// פונקציה שמזדקנת את המשתמש בשנה
    pub fn birthday(&mut self) {
        self.age += 1;
    }

    /// פונקציה שמכבה את המשתמש
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

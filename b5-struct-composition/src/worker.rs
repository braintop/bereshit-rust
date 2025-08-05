// src/worker.rs

use crate::user::User;

pub struct Worker {
    pub user: User, // קומפוזיציה (הרכבה)
    pub job_title: String,
}
impl Worker {
    pub fn new(username: String, job_title: String) -> Self {
        Worker {
            user: User::new(username),
            job_title,
        }
    }

    pub fn print(&self) {
        self.user.print();
        println!("Job title: {}", self.job_title);
    }
}

use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();
    println!("New ID: {}", id);
}

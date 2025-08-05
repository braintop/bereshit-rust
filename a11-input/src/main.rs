use std::io;
fn create_vec(){
    let mut numbers: Vec<i32> = Vec::new();
    for i in 0..5 {
        numbers.push(i*10);
    }
    println!("{:?}", numbers);
}
fn main() {
    create_vec();
}




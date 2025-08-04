use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("שגיאה בקריאת הקלט");

    let num: i32 = input.trim().parse().expect("נא להזין מספר תקין");

    println!("input is : {}", num);
}




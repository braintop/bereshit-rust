
fn main() {
    let x = 4;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }


    let age = 20;
    match age {
        0 => println!("Zero"),
        1..=18 => println!("Child"),
        19..=64 => println!("Adult"),
        _ => println!("Senior"),
    }


    let day = "Sunday";
    match day {
        "Sunday" | "Monday" => println!("Start of the week"),
        "Friday" | "Saturday" => println!("End of the week"),
        _ => println!("Midweek"),
    }

    //match gourd of a fruit
    let number = 10;
    match number {
        n if n % 2 == 0 => println!("Even"),
        n if n % 2 != 0 => println!("Odd"),
        _ => println!("Other"),
    }
    
}

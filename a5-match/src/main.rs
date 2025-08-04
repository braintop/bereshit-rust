
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

    let number = 90;

    match number {
        90..=100 => println!("A"), // .. means range 90-100
        80..=89 => println!("B"),
        70..=79 => println!("C"),
        60..=69 => println!("D"),
        0..=59 => println!("F"),
        _ => println!("Invalid grade"),
    }

    
}

//init grade with number 1-100
//if grade is 90-100 print "A"
//if grade is 80-89 print "B"
//if grade is 70-79 print "C"
//if grade is 60-69 print "D"
//if grade is 0-59 print "F"
//if grade is not a number print "Invalid grade"




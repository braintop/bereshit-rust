mod orir;
fn check_positive(num: i32) -> Result<i32, String> {
    if num > 0 {
        Ok(num)
    } else {
        Err(String::from("Number is not positive"))
    }
}

fn main() {
    let result1 = check_positive(10);
    let result2 = check_positive(-5);

    match result1 {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    match result2 {
        Ok(value) => println!("Success: {}", value),
        Err(error) => println!("Error: {}", error),
    }
    println!("{}", math::add(1, 2));
}
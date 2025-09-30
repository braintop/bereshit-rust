#[derive(Debug)]
enum MathError {
    DivideByZero,
    NegativeRoot,
}

fn calc(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        return Err(MathError::DivideByZero);
    }
    if a < 0 {
        return Err(MathError::NegativeRoot);
    }
    Ok(a / b)
}

fn main() {
    // דוגמה 1 – חילוק רגיל
    match calc(10, 2) {
        Ok(val) => println!("Result = {}", val),
        Err(e) => println!("Error: {:?}", e),
    }

    // דוגמה 2 – חילוק באפס
    match calc(10, 0) {
        Ok(val) => println!("Result = {}", val),
        Err(e) => println!("Error: {:?}", e),
    }

    // דוגמה 3 – מספר שלילי
    match calc(-10, 2) {
        Ok(val) => println!("Result = {}", val),
        Err(e) => println!("Error: {:?}", e),
    }
}

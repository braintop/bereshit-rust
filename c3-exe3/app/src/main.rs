fn safe_sqrt(x:f64) -> Result<f64, String> {
    if x < 0.0 {
        return Err("Cannot square root of a negative number".to_string());
    }
    Ok(x.sqrt())
}

fn main() {
    let result = safe_sqrt(16.0);
    match result {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
    println!("--------------------------------");
    match safe_sqrt(-4.0) {
        Ok(val) => println!("√-4 = {}", val),
        Err(e) => println!("Error: {}", e),
    }

}
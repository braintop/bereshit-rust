// פונקציה שמבצעת חלוקה עם טיפול בשגיאה
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// פונקציה חדשה שמשתמשת ב-? כדי "להעביר הלאה" את השגיאה
fn double_divide(a: i32, b: i32) -> Result<i32, String> {
    let result = safe_divide(a, b)?; // אם יש Err → חוזר מיד
    Ok(result * 2)
}

fn main() {
    // דוגמה עם חילוק תקין
    match double_divide(10, 2) {
        Ok(val) => println!("Result = {}", val),
        Err(e) => println!("Error: {}", e),
    }

    // דוגמה עם חלוקה באפס
    match double_divide(10, 0) {
        Ok(val) => println!("Result = {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

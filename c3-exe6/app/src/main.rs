fn first_element(nums: Vec<i32>) -> Result<i32, String> {
    if nums.is_empty() {
        Err("empty list".to_string())
    } else {
        Ok(nums[0])
    }
}

fn main() {
    // דוגמה עם וקטור לא ריק
    match first_element(vec![10, 20, 30]) {
        Ok(val) => println!("First element = {}", val),
        Err(e) => println!("Error: {}", e),
    }

    // דוגמה עם וקטור ריק
    match first_element(vec![]) {
        Ok(val) => println!("First element = {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

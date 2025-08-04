
// i32 – מספר שלם

// f64 – מספר עשרוני


// bool – נכון/שקר


// char – תו בודד


// &str – מחרוזת




fn main() {
    let age: i32 = 20;
    let name: &str = "John";
    let is_student: bool = true;
    let height: f64 = 1.75;
    let letter: char = 'A';
    let message: &str = "Hello, world!";
    println!("Age: {}", age);
    println!("Name: {}", name);
    println!("Is student: {}", is_student);
    println!("Height: {}", height);
    println!("Letter: {}", letter);
    println!("Message: {}", message);
}

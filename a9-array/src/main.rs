
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];
    for number in numbers {
        println!("{}", number);
    }

    numbers[0] = 1000;
    println!("first number: {}", numbers[0]);

    let arr = [10, 20, 30];
    for i in 0..arr.len() {
        println!("arr[{}] = {}", i, arr[i]);
    }

    let mut arr2: [i32; 5] = [0; 5];
    for i in 0..arr2.len() {
        println!("arr2[{}] = {}", i, arr2[i]);
    }

    let mut arr3 = [0; 5];
    for i in 0..arr3.len() {
        println!("arr3[{}] = {}", i, arr3[i]);
    }

    // 🧩 שלב 6: תרגול לתלמידים
    // כתבו פונקציה שמקבלת מערך של 5 מספרים ומחזירה את הסכום שלהם.
    
    // צרו מערך של 10 מספרים ואתרו את הערך המקסימלי.
    
    // כתבו פונקציה שמדפיסה רק את המספרים הזוגיים במערך.
    
    



}




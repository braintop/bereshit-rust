fn average1(numbers: Vec<i32>) -> f64 {
    let sum: i32 = numbers.iter().sum();
    let count = numbers.len();
    if count == 0 {
        return 0.0;
    }
    return sum as f64 / count as f64;
}

fn average2(numbers: Vec<i32>) -> f64 {
    let mut sum = 0;
    let mut count = 0;
    for num in numbers {
        sum += num;
        count += 1;
    }
    if count == 0 {
        return 0.0;
    }
    return sum as f64 / count as f64;
}


fn main() {
    // צור וקטור ריק של String
    let mut names: Vec<String> = Vec::new();

    names.push("John".to_string());
    names.push("Jane".to_string());
    names.push("Jim".to_string());
    names.push(String::from("sara"));

    println!("length: {}", names.len());
    names.remove(1);

    let first = String::from("שלום");
    let second = String::from(" עולם");

    let result = first + &second;
    println!("{}", result);

    let numbers = vec![15, 25, 35];
    let avg = average2(numbers);
    println!("The average is: {}", avg); // The average is: 25


}




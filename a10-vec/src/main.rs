fn average(numbers: Vec<i32>) -> f64 [
    
]

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

}




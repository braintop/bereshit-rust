// Import the `json!` macro from the serde_json crate
// This macro makes it easy to create JSON values inline
use serde_json::json;

fn main() {
    // Create a JSON object using the `json!` macro
    // The result is of type `serde_json::Value`
    let data = json!({
        "id": 1,
        "name": "Laptop"
    });

    // Convert the JSON value to a String and print it
    // Output: {"id":1,"name":"Laptop"}
    println!("{}", data.to_string());
}

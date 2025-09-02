

fn main() {


    let mut words = vec!["rust", "is", "great"];
    words.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("{:?}", words); // ["is", "rust", "great"]
        }

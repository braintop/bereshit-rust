use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file("src/data.txt") {
        Ok(content) => {
            println!("File content:");
            println!("{}", content);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let mut file_path = String::new();

    if io::stdin().read_line(&mut file_path).is_err() {
        println("failure");
        return;
    }

    let trimmed_path = file_path.trim();
    if trimmed_path.is_empty() {
        println!("failure");
        return;
    }

    let p = Path::new(trimmed_path);
    if p.is_dir() {
        println!("failure");
        return;
    }
    let mut file = match File::open(trimmed_path) {
        Ok(f) => f,
        Err(_) => {
            println!("failure");
            return;
        }
    };
    let mut buffer = [0u8; 1];
    match file.read(&mut buffer) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}

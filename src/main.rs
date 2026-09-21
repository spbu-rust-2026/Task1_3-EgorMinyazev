use std::fs;
use std::io;

fn main() {
    let mut path = String::new();

    io::stdin().read_line(&mut path).expect("Error");

    let path = path.trim();

    if fs::read_to_string(path).is_ok() {
        println!("success");
    } else {
        println!("failure");
    }
}

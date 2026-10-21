use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("Could remove File");
    println!("File has been removed.");
}

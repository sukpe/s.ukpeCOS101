use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;

fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").unwrap();
    file.write_all("\nHello Class".as_bytes()).unwrap();
    file.write_all("\nThis is the  appendage to the document".as_bytes()).unwrap();
    println!("Append succeded");

    let mut show = String::new();
    let mut contains = std::fs::File::open("data.txt").unwrap();
    contains.read_to_string(&mut show).unwrap();
    println!("{}",show );

}

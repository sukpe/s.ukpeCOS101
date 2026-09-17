//building a rust program to output users name and age

use std::io;

fn main() {
    println!("\n Students Information Management System!");

//input of their name
    println!("\n Please input your name.");
    let mut name = String::new();

    io::stdin()
    .read_line(&mut name)
    .expect("Please this name input is invailed");

    println!("\n Hello {} Welcome to my World!", name);

//input of their age
    println!("\n Please input your age.");
    let mut age = String::new();

    io::stdin()
    .read_line(&mut age)
    .expect("Can't read this input.");

    let age:u8 = age.trim().parse().expect("This is not an integer.");
    println!("\n WOW!!, You are {} years old", age);

}

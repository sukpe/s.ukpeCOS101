//Rust Party age veriication pass

use std::io;

    fn main() {
        let mut input1 = String::new();
        let mut input2 = String::new();

        println!("Please enter your name: ");
        io::stdin().read_line(&mut input1).expect("This is not a valid entry.");

        println!("Please enter your age: ");
        io::stdin().read_line(&mut input2).expect("This is not a vaule.");
        let age:u8 = input2.trim().parse().expect("This is not a valid value");

        if age >= 18 {
        println!("Welcome to the party {}!", input1);
    } else {
        println!("Ahh ah checky boy!, you are not old enough to enter this party {}", input1);

        let till_age:u8 = 18 - age;
        println!("{} Sorry but you have to wait {} more years to enter this party, Thank you for your time!",input1,till_age );

    }
}   


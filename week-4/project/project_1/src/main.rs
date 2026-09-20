//A rust program to solve quadratic equaion of the 3 given  constants a, b, and c

use std::io;

fn main() {
//Get users for interaction.
println!("Before we start I would like to know your name?");
let mut name_t = String::new();
io::stdin().read_line(&mut name_t).expect("Failed to read your input, please try again.");
let name = name_t.trim();

//starting message
    println!("Hello {} Welcome to a rust powered quadratic equation solver", name);

//lets begin    
            let mut input1 = String::new();
            let mut input2 = String::new();
            let mut input3 = String::new();
            
            //The value of a
            println!("Alright lets get started!");
            println!("\nPlease input the of the coefficent of x^2 (This is the value infront of x^2)");
            io::stdin().read_line(&mut input1).expect("Failed to read your input. Please try again.");
            let a:f64 = input1.trim().parse().expect("Failed to process your input. Please try again.");

            //the value of b
            println!("\nPlease input the of the coefficent of x (This is the value infront of x)");
            io::stdin().read_line(&mut input2).expect("Failed to read your input. Please try again.");
            let b:f64 = input2.trim().parse().expect("Failed to process your input. Please try again.");

            //the value of c
            println!("\nPlease input the of the value of the constant (This is the value the has no x)");
            io::stdin().read_line(&mut input3).expect("Failed to read your input. Please try again.");
            let c:f64 = input3.trim().parse().expect("Failed to process your input. Please try again.");

            //Solving the quadratic equation
            let d = b*b - 4.0*a*c;
            
            if a !=0.0 { println!("The discriminat is {}",d);}

            if d > 0.0 && a != 0.0 {
                println!("Your equation has two distinct roots");}
            else if d < 0.0 && a != 0.0 {
                println!("Your equation has no real roots");}
            else if d == 0.0 && a != 0.0 {
                println!("Your equation has exactly only one real root");}
                else if a == 0.0 {
                    println!("{}! This is a linear equation!", name);
                }
//choice of learning the  steps
            let mut choice = String::new();
            println!("So {} Would you like me to proceed in solving for the real roots?", name);
            println!("Please y/n to input your choice");
            io::stdin().read_line(&mut choice).expect("Failed to read your input. Please try again.");
            if choice.trim().to_lowercase() == "y" { 
                //checking if it is a quadratic equation
                if a == 0.0{
                    println!("I am so sorry {} but this is a linear equation!",name);
                    println!("Have a great day!");
                }

            //solving the quadratic equation
                else {
                    if d > 0.0 || d == 0.0 && a != 0.0{
                    let e = (-b + d.sqrt())/(2.0*a);
                    let f = (-b - d.sqrt())/(2.0*a);
                    println!("The root of the quadratic equation are {} or {}",e ,f );}
                else if d < 0.0 && a != 0.0{
                    let real = -b/(2.0*a);
                    let img = (d.abs().sqrt())/(2.0*a);
                    println!("The root of the quadratic equation are {}+{}i or {}-{}i",real, img, real, img );
                    println!("Take note that  there is an i at the end of the values show it is an imaginary number!");
                } 
                }}
            

            else if choice.trim().to_lowercase() == "n" {
                println!("Alright have a great day {}!", name);}
                else {
                println!("There was an issue with you input {}", name);
                println!("Please endevour that u inputted y/n, nothing orther than y/n. Thank you")
            }
        }

//A rust program to calculate the area of a triangle

use std::io;

fn main()
{   
    println!("\n Welcome to the Triangle Area Calculator.");

//user values
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\n Please input the value of the first side of the triangle.");
    io::stdin().read_line(&mut input1).expect("This input can't be read.");
    let a:f32 = input1.trim().parse().expect("This is not a real  value.");

    println!("\n Please input the value of the second side of the triangle.");
    io::stdin().read_line(&mut input2).expect("This input can't be read.");
    let b:f32 = input2.trim().parse().expect("This is not a real value.");

    println!("\n Please input the value of the third side of the triangle.");
    io::stdin().read_line(&mut input3).expect("This input can't be read,");
    let c:f32 = input3.trim().parse().expect("This is not a real value.");

//calculation of the area using heron's formula
    let s:f32 = (a+b+c)/2.0;    //perimetere of a triangle
    let mut area = s*(s-a)*(s-b)*(s-c);
    area = area.sqrt(); 

//Users area output
    println!("\n The area of the triangle is {}",area);

}

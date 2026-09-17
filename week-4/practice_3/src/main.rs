//A rust program to calculate the area of a triangle

use std::io;

fn main()
{   
    println!("\n Welcome to the Triangle Area Calculator.");

//user values
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\n What is the triangles base?");
    io::stdin().read_line(&mut input1).expect("This input can't be read.");
    let base:f32 = input1.trim().parse().expect("This is not a real  value.");

    println!("\n What is the triangle height?");
    io::stdin().read_line(&mut input2).expect("This input can't be read.");
    let height:f32 = input2.trim().parse().expect("This is not a real value.");

//Condition
    if base > 0.0 {
        let area = (height*base)/2.0;   

        //Users area output
    println!("\n The area of the triangle is {}",area);
}
}

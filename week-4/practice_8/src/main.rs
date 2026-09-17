use std::io;

fn main() {
    //while true
    println!("What do you want as your value of x?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read your input");
    let mut x:i8 = input1.trim().parse().expect("Failed to input");
    loop{
        x+=1;
        println!("x = {}", x);

        if x == 15 {
            break;
        }
    }
}

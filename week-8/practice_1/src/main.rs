fn main() {
    //Using Vec::new()
    let v : Vec<i32> = Vec::new();

    //printing the size
    println!("\nThe length of Vec::new() is: {}", v.len());

    //Using macro
    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"];

    //printing the size of the vector
    println!("\nThe length of vec macro is: {}", v.len());
}

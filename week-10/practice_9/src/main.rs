struct Rectangle{
    width:u32, height:u32
}
impl Rectangle{
    //the  self keyword allows the function to have access to the struct and the values in the struct
    fn area(&self)->u32{
        self.width*self.height
    }
}


fn main() {
    let small = Rectangle{
        width: 10,
        height: 20
    };
    println!(" width is {}\n height is {}\n area of the rectangle is {}",small.width,small.height,small.area())
}

fn add_one(e: &mut u8) {
    *e += 1;
}

fn main() {
    let mut i = 3;
    println!("i before the function{}", i);
    add_one(&mut i);
    //anywhere u see "&" it is borrow posing therefore the ownership hasnt been moved 
    println!("i after the function{}", i);
}

fn main() {
    let mut colours = ["red","green","yellow","white'"];

    println!("\nOriginal array = {:?}",colours );

    let slice_colours = &mut colours[1..3];

    println!("First slice = {:?}",slice_colours );

    slice_colours[1] = "purple";

    println!("Changed slice = {:?}",slice_colours );
}

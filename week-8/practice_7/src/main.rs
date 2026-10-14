fn main() {
    let datatype_tuple: (&str, f32, u8 ) = ("Rust", 3.14, 100);
    println!("Tuple contents = {:?}", datatype_tuple);

    let no_datatype_tuple = ("Rust", '#', 100, 1.234);
    println!("Tuple contents = {:?}", no_datatype_tuple);


    println!("Value at index 1 = {}", datatype_tuple.0 );

    println!("Value at index 2 = {}", datatype_tuple.1 );

    println!("Value at index 3 = {}", datatype_tuple.2 );

}

fn main() {
    let mut fees:u32 = 25_000; //here the "mut" has been added and no error will occur because fees is now able to change 
    println!("fees is {}", fees);

    fees = 35_000;
    println!("fees changed is {}", fees);
}

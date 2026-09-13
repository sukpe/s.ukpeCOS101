fn main() {
    let fees:u32 = 25_000; //This is immutable which means that the value assigned to can't be changed unless the "mut" tag is added to it
    println!("fees is {}", fees);

    fees = 35_000;
    println!("fees changed is {}", fees); 
}

fn main() {
    let special_character = '@'; //default set as char type
    let alphabet:char = 'B';
    let surname = "Bolaji";     //default set as string type
    let first_name:&str = "Michael"; //to define a string use &str
    let middle_name:&str = "Gboyega"; //&str is the immautable version of defining a string   

    println!(" ");
    println!("Special character: {}",special_character);
    println!("Alphabet: {}",alphabet);
    println!("Name: {} {} {}",surname, middle_name, first_name);
}

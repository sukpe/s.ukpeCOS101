use std::fs::File;
use std::io::Write;

fn main() {
    //Vectors
    let head = " Student Names, Matric Number, Department, Level ";
    let name = vec![" Oluchi Mordi "," Adams Aliyu " ," Shania Bolade "," Adekunle Gold "," Blanca Edemoh "];
    let matric = vec![" ACC10211111 "," ECO10110101 "," CSC10328828 "," EEE11020202 "," MEE10202001 "];
    let dept = vec![" Accounting "," Economics "," Computer "," Electrical "," Mechanical "];
    let lvl = vec![" 300 "," 100 " ," 200 "," 200 "," 100 "];

    //Writing it to the file.
    println!("{}",head);
    let mut file = File::create("SMIS.csv").unwrap();
    file.write_all(head.as_bytes()).unwrap();
    //Using a for loop to write for every index
  for i in 0..name.len(){
    let row = format!("\n{},{},{},{}",name[i], matric[i],dept[i], lvl[i]);  
    file.write_all(row.as_bytes()).unwrap();

    println!("{}",row);
}

println!("Success!");
}

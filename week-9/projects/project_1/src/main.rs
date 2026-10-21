use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

fn main() {
    //remove any old version of the file
    std::fs::remove_file("NB_PLC.csv").expect("Failed to remove.");

    println!("The categories of drinks for the Nigerian Breweries Plc.\nWould You like to add another categorie??\nYes or No.");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input.");

    if choice.trim().to_lowercase().as_str() == "yes"{
        let head = "Lager,Stout,Non-Achoholic";
        let row1 = "33 Expert,Legend,Maltina";
        let row2 = "Desperados,Turbo King,Amstel Malta";
        let row3 = "Goldberg,Williams,Malta Gold";
        let row4 = "Gulder,,Fayrouz";
        let row5 = "Heineken,,";
        let row6 = "Star,,";
    
        //Creating the file
        let mut file = File::create("NB_PLC.csv").expect("Failed to create file.");
        file.write_all(head.as_bytes()).expect("Failed to write");
        file.write_all(row1.as_bytes()).expect("Failed to write"); 
        file.write_all(row2.as_bytes()).expect("Failed to write");
        file.write_all(row3.as_bytes()).expect("Failed to write");
        file.write_all(row4.as_bytes()).expect("Failed to write");
        file.write_all(row5.as_bytes()).expect("Failed to write");
        file.write_all(row6.as_bytes()).expect("Failed to write");
    
        let mut file_ap = OpenOptions::new().append(true).open("NB_PLC.csv").expect("Failed to append");
        println!("Input what you would like to append");
        let mut ap_in = String::new();
        io::stdin().read_line(&mut ap_in).expect("Failed to read");
        file_ap.write_all(ap_in.as_bytes()).expect("Failed to write.");

        //reading the file 
        let mut file_re = File::open("NB_PLC.csv").expect("Failed to create file.");
        let mut sheet = String::new();
        file_re.read_to_string(&mut sheet).expect("Failed to read file.");
        println!("{}",sheet);

    }else
    {   let head = "\nLager,Stout,Non-Achoholic";
        let row1 = "\n33 Expert,Legend,Maltina";
        let row2 = "\nDesperados,Turbo King,Amstel Malta";
        let row3 = "\nGoldberg,Williams,Malta Gold";
        let row4 = "\nGulder,,Fayrouz";
        let row5 = "\nHeineken,,";
        let row6 = "\nStar,,";
    
        //Creating the file
        let mut file = File::create("NB_PLC.csv").expect("Failed to create file.");
        file.write_all(head.as_bytes()).expect("Failed to write");
        file.write_all(row1.as_bytes()).expect("Failed to write"); 
        file.write_all(row2.as_bytes()).expect("Failed to write");
        file.write_all(row3.as_bytes()).expect("Failed to write");
        file.write_all(row4.as_bytes()).expect("Failed to write");
        file.write_all(row5.as_bytes()).expect("Failed to write");
        file.write_all(row6.as_bytes()).expect("Failed to write");
    
        //reading the file 
        let mut file_re = File::open("NB_PLC.csv").expect("Failed to create file.");
        let mut sheet = String::new();
        file_re.read_to_string(&mut sheet).expect("Failed to read file.");
        println!("{}",sheet);}
}

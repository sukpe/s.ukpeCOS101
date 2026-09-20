//The Incentive Calculator

use std::io;

fn main() {
    println!("Welcome to the rust incentive calculator");
    println!("This is used to calculate and make decision as a great business owner");
    println!("please may i get to know What your name is");
    let mut name_t = String::new();
    io::stdin().read_line(&mut name_t).expect("Failed to read your input please try again.");
    let name = name_t.trim();

    //inputs
    println!("Thank you for choosing us {}",name );
    println!("Can you tell me the age of this your expert empolyee");
    let mut age_i = String::new();
    io::stdin().read_line(&mut age_i).expect("Failed to read your input. Please try again");
    let age:u8 = age_i.trim().parse().expect("Failed to process your input. Please try again.");

    println!("Alright is the empolyee 'Experienced' or 'Not Experienced' ");
    let mut exp_i = String::new();
    io::stdin().read_line(&mut exp_i).expect("Failed to read your input. Please try again");
    let exp = exp_i.trim().to_lowercase();
    // the decision making 
    if exp =="experienced"{if age >= 40  {
            let sal:f32 = 1_560_000.00;
            println!("For someone who is {} and {} I feel their salary should be {}",age,exp,sal);
   } else if age <= 38 && age >= 30 {
            let sal:f32 = 1_480_000.00;
            println!("An employee who is {} and is also {} shold be given a salary of {} only",age,exp,sal);
   }else if age < 28 {
            let sal:f32 = 1_300_000.00;
            println!("To be honest with you and to do my task of making your business follow i would advice that for someone who is {} and is {} should be paid {}",age,exp,sal);
   
   }
    }
   else{
            let sal:f32 = 100_000.00;
            println!("This person {} and regardless the age i beileve that salary should be {}",exp, sal);}
   println!("Thank you for choosing me {} come again soon!",name );

}

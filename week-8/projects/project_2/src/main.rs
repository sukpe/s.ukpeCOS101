use std::io;

fn main() {

    let mut candid_name: Vec<String>= Vec::new();
    let mut candid_exp: Vec<u8> =  Vec::new();

    println!("Welcome to the EY Nigeria Interviewing platform\nHow many interviewees do we have today??");
    let mut interviewees_in = String::new();
    io::stdin().read_line(&mut interviewees_in).expect("Failed to read");
    let interviewees: u8 = interviewees_in.trim().parse().expect("Process Failed");

    if interviewees == 0 {
        println!("Today was peaceful, No interviewees");
    } else {

        for i in 0..interviewees {
            println!("Welcome interviewee number {}, Can I know what your name is??", i + 1);
            let mut name_in = String::new();
            io::stdin().read_line(&mut name_in).expect("Failed to read");
            let name = name_in.trim().to_string();

            println!("Alright {}, How many years of programming experience do you have??", name);
            let mut exp_in = String::new();
            io::stdin().read_line(&mut exp_in).expect("Failed to read");

            let exp: u8 = exp_in.trim().parse().expect("Process Failed");

            candid_name.push(name);
            candid_exp.push(exp);
        }

        let mut max_exp = candid_exp[0];
        let mut index = 0;

        

        for i in 1..candid_exp.len(){
                    if max_exp == candid_exp[i]{
                        println!("Equal experience pick anyone.");

                    }else if candid_exp[i] > max_exp {
                        max_exp = candid_exp[i];
                        index = i;
                    
                    }
                   
                }
        let best_name = &candid_name[index];
        println!("The people we interviewed today: {:?}", candid_name);
        println!("Their respective years of experience: {:?}", candid_exp);
        println!("\nBy going through the interviewees of today, {} has the highest years of experience with {} years of programming experience. Therefore I believe {} is the best fit for the job.", best_name, max_exp, best_name);
        
    }
}

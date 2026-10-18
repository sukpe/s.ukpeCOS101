use std::io;
    
fn main() {
    let public_servant: Vec<&str> = vec!["APS 1-2","APS 3-5","APS 5-8", "EL1 8-10", "EL2 10-13", "SES" ];
    //Greeting the user and getting first input their name
    println!("Welcome user to the rust APS level checker\nhere are the following APS levels {:?}",public_servant);
    println!("I can help you determine your APS level is you can just help me answer some basic questions"); 
    let mut in_name = String::new();
               println!("Please enter your name:");
       loop {io::stdin().read_line(&mut in_name).expect("Failed to read input");
                      let name = in_name.trim();
               println!("{}",name);
               if !name.is_empty() && name.chars().any(|c| !c.is_numeric()){
                break;
               }else {
                println!("Your name cant be blank or contain a value!");
               }
           }
        
    //asking which category they belong
    let mut cate = String::new();
    println!("Which of the following categories do you fall under?");
    println!("\n1. Law\n2. Offices\n3. Academic\n4. Teaching");
    println!("Please type in the category for a drop down menu:");
    io::stdin().read_line(&mut cate).expect("Failed to read input");
    match cate.trim().to_lowercase().as_str(){
           "law" => {law()}
           "office" => {office()}
           "academic" => {academ()},
           "teaching" => {teach()},
           _=> println!("Your input is not registered please check your spelling and type what is above."),
       };
}
fn law(){
    let public_servant: Vec<&str> = vec!["APS 1-2","APS 3-5","APS 5-8", "EL1 8-10", "EL2 10-13", "SES" ];
    let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2","Senior Associate 3-4","Patner"];
    println!("Welcome to the lawyers class where all the following categories belong:\n{:?}",lawyer);
    let mut law_cat = String::new();
    io::stdin().read_line(&mut law_cat).expect("Failed to read input");
    
   let laws = match law_cat.trim().to_lowercase().as_str(){
    "paralegal"=> 0,
    "junior associate"=>1,
    "associate"=>2,
    "senior associate 1-2" => 3,
    "senior associate 3-4" => 4,
    "patner" => 5,
    _=> 6,
   };
   let law_aps:&str = public_servant[laws];
   println!("{}", law_aps);
}
fn office(){
    let public_servant: Vec<&str> = vec!["APS 1-2","APS 3-5","APS 5-8", "EL1 8-10", "EL2 10-13", "SES" ];
    let office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager","Director","CEO"];
    println!("Welcome to the lawyers class where all the following categories belong:\n{:?}",office_admin);
    let mut office_cat = String::new();
    io::stdin().read_line(&mut office_cat).expect("Failed to read input");
   
   let offices = match office_cat.trim().to_lowercase().as_str(){
    "intern"=> 0,
    "administrator"=>1,
    "senior administrator"=>2,
    "office manager" => 3,
    "director" => 4,
    "ceo" => 5,
    _=> 6,
   };
   let office_aps:&str = public_servant[offices];
   println!("{}", office_aps);
}
fn teach(){
    let public_servant: Vec<&str> = vec!["APS 1-2","APS 3-5","APS 5-8", "EL1 8-10", "EL2 10-13", "SES" ];
    let teach = vec!["Placement","Classroom Teacher","Senior Teacher","Leading Teacher","Deputy Princepal","Princepal"];
    println!("Welcome to the lawyers class where all the following categories belong:\n{:?}",teach);
    let mut teach_cat = String::new();
    io::stdin().read_line(&mut teach_cat).expect("Failed to read input");
    let teachs = match teach_cat.trim().to_lowercase().as_str(){
    "placement"=> 0,
    "classroom teacher"=>1,
    "senior teacher"=>2,
    "leading teacher" => 3,
    "deputy princepal" => 4,
    "princepal" => 5,
    _=> 6,
   };
   let teach_aps:&str = public_servant[teachs];
   println!("{}", teach_aps);  
}
fn academ(){
    let public_servant: Vec<&str> = vec!["APS 1-2","APS 3-5","APS 5-8", "EL1 8-10", "EL2 10-13", "SES" ];
    let academic = vec!["-","Research Assitant","PhD Candidate","Post-Doc Researcher","Senior lecturer","Dean",];
    println!("Welcome to the lawyers class where all the following categories belong:\n{:?}",academic);
    let mut academ_cat = String::new();
   io::stdin().read_line(&mut academ_cat).expect("Failed to read input");
    
    let academia = match academ_cat.trim().to_lowercase().as_str(){
    "-"=> 0,
    "research assitant"=>1,
    "phd Candidate"=>2,
    "post-doc researcher" => 3,
    "senior lecturer" => 4,
    "dean" => 5,
    _=> 6,
   };
   let academ_aps:&str = public_servant[academia];
   println!("{}", academ_aps);
}



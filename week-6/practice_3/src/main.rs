fn main(){
    let name1 = "Ayomide Adesokan";
    println!("My name is {}",name1);
    
    //find and replace
    let name1 = name1.replace("Ayomide","Adebare"); //Note using the .replace() it doesnt need the variable to be mutable 
    println!("You can also call me {}",name1);
    let faculty = "Faculty of Science and Technology";
    
    //find and replace
    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}", school);
    
}

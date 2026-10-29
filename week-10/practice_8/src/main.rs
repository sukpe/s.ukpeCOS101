//declare a structure
struct Employee {
    ceo:String,
    company:String,
    age:u32
}
fn main() {
    //initialize a structure
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Satya Nadella"),
        age:56
    };
    let emp2 = Employee{
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichai"),
        age:51
    };
    //pass emp1 and emp2 to display()
    //due to the presence of the two Strings, which is stored in the heap, ownership is transfered to the function.
    display(emp1);
    display(emp2);
}
// fetch values of specific structure fields using the
// operator and print it to the console
//A  struct created a data type called employee
fn display(emp:Employee){
    println!("Name is {} company is {} age is
    {}",emp.ceo,emp.company,emp.age);
}

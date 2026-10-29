//A struct can create its own data type. now there is a data type called employee.
//if a single data type stored in a heap is in the sturct and the struct is passed by value to a function then ownership is passed to the function.
struct Employee {
    name: String,
    company: String,
    age: u32,
}

fn main() {
    let emp1 = Employee {
        company: String::from("Enrst & Young"),
        name: String::from("Ebibiong Jessica"),
        age: 25,
    };
    println!("Name = {} \n", emp1.name);
    println!("Company = {} \n", emp1.company);
    println!("Age = {}", emp1.age);
}

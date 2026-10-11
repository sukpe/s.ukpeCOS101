use std::io;

// Pi 
fn pi() -> f64 {
    let pi = 22.0 / 7.0;
    return pi;
}

// input
fn input_num() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let number: f64 = input.trim().parse().expect("Input not a number");
    if input.is_empty(){
        println!("This is empty try again");
    }else {
    println!("{}",number );
};return number;
}

fn trap_area() -> f64 {
    println!("input the trapezium:");
    let height = input_num();
    println!("input the base:");
    let base_1 = input_num();
    println!("input the second base:");
    let base_2 = input_num();
    let area_trap = height / 2.0 * (base_1 + base_2);
    return area_trap;
}

fn rhom_area() -> f64 {
    println!("input the first diagonal:");
    let diagonal1 = input_num();
    println!("input the second diagonal:");
    let diagonal2 = input_num();
    let area = 1.0 / 2.0 * diagonal1 * diagonal2;
    return area;
}

fn para_area() -> f64 {
    println!("input the parallelogram's base:");
    let base = input_num();
    println!("input the perpendicular height:");
    let altitude = input_num();
    let area = base * altitude;
    return area;
}

fn cube_surf_area() -> f64 {
    println!("input the length of one side of the cube:");
    let side = input_num();
    let surface_area = 6.0 * side * side;
    return surface_area;
}

fn cylin_volume() -> f64 {
    println!("input the radius of the cylinder:");
    let radius = input_num();
    println!("input the height of the cylinder:");
    let height = input_num();
    let volume = pi() * radius * radius * height;
    return volume;
}

fn main() {
    println!("Welcome to the rust powered Shape Calculator!");
    println!("Please input your name");
    let mut name_in = String::new();
    io::stdin().read_line(&mut name_in).expect("Input Failed ");
    let name = name_in.trim();
    println!("What would you like to work out today? {}", name);
    println!("1 Trapezium (area)");
    println!("2 Rhombus (area)");
    println!("3 Parallelogram (area)");
    println!("4 Cube (Total surface area)");
    println!("5 Cylinder (volume)");
    println!("Input a number from above:");

    loop {
        let mut choice_text = String::new();
        io::stdin().read_line(&mut choice_text).expect("Failed to read input");
        let choice: u32 = choice_text.trim().parse().expect("Input not an integer");
    
        if choice == 1 {
            let area = trap_area();
            println!("The area of your trapezium is {}", area);
    
        } else if choice == 2 {
            let area = rhom_area();
            println!("The area of your rhombus is {}", area);
        } else if choice == 3 {
            let area = para_area();
            println!("The area of your parallelogram is {}", area);
        } else if choice == 4 {
            let surf_area = cube_surf_area();
            println!("The surface area of your cube is {}", surf_area);
        } else if choice == 5 {
            let volume = cylin_volume();
            println!("The volume of your cylinder is {}", volume);
        } 

        if choice <=1 && choice >= 5 {
            break;
        } else {
            println!("Oops, {} isn't on the menu. Run the program again and pick 1 to 5.", choice);
        } 
    }
}

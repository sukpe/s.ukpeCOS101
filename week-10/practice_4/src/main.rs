fn main() {
    let v = vec![15,25,35,45,55];
    print_vector(v);
       
}
fn print_vector(v:Vec<i32>){
    println!("inside your print_vector {:?}",v );
    println!("{}",v[0]);
}
fn main() {
    let v = vec![101,250,330,400];
    //v is the owner of the objects in this heap

    //only a single variable can own the heap memory at a given time
    let v2 = v;
    //in this case 2 variables want to own the heap value
    //this is not allowed in rust

    //rust can easily detect stuff like this for 2 variables are pointing to the same heap
    println!("{:?}",v2)

}

fn main() {
	let p:f64 = 1000.0;
	let r:f64 = 1.0;
	let t:f64 = 2.0;

	//simple interest
	let si = ((p*r*t)/100.0);
	println!("Simple Interest is {}", si);
	let a = si + p;
	println!("Amount is {}", a );
	
}
fn main() {
	//stating our variables
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	//calculating the depriciation
	let dep = p*(1.0 -(r/100.0)).powf(t);

	//display output
	println!("In three years the value of the TV set should become {} ", dep);
}

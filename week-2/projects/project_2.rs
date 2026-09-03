fn main(){
	// The list of the amount of the different products
	let t_a:f64 = 450_000.00;
	let m_a:f64 = 1_500_000.00;
	let h_a:f64 = 750_000.00;
	let d_a:f64 = 2_850_000.00;
	let a_a:f64 = 250_000.00;

	// The list of the quanity of the different products that where sold
	let t_q:f64 = 2.0;
	let m_q:f64 = 1.0;
	let h_q:f64 = 3.0;
	let d_q:f64 = 3.0;
	let a_q:f64 = 1.0;

	// getting the actual individual sum
	let t_s = t_q*t_a;
	let m_s = m_q*m_a;
	let h_s = h_q*h_a;
	let d_s = d_q*d_a;
	let a_s = a_q*a_a;

	// Finding the sum total
	let sum = t_s + m_s + h_s + d_s + a_s;

	// Finding the sum of quantity
	let sum_q = t_q + m_q + h_q + d_q + a_q;

	// Finding the average
	let avg = sum/sum_q;

	//The Output
	println!("The total sum of everything is {}", sum);

	println!("The Average Of Everything is {}",  avg);


}
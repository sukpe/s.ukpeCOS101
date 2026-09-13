fn main() {
   let result = 10.00;        //f64 by default
   let interest:f32 = 8.35;   // single precision: which means less the f64 due to small amount of storage being taken
   let cost:f64 = 15000.600;  //double precision: which means more accomodation for more decimal places making a more precises output

   println!("result value is {}",result);
   println!("interest is {}",interest);
   println!("cost is {}",cost);
}

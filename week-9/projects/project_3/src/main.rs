use std::io::Write;
use std::fs::File;

fn main() {
    //vectors
    let head = "Commissioner,   Ministry,    Zone";
    let com  = vec![" Aigbogun Alamba Daudu "," Murtala Afeez Bendu "," Okorocha Calistus Ogbona "," Adewale Jimoh Akanbi "," Osazuwa Faith Etieye "];
    let min = vec![" Internal Affairs "," Justice "," Defense "," Power & Steel "," Petroleum "];
    let zone = vec![" South West "," North East "," South South "," South West "," South East "];

    println!("Here are the seperate tables \nCommissioner :{:?},\nMinistry: {:?},\nZone: {:?}",com,min,zone);

    //writing to the files
    println!("\n{}",head);
    let mut file = File::create("EFCC_Merge.csv").unwrap();
    file.write_all(head.as_bytes()).unwrap();
  //using of the for loop to write every index to the file and pritn
  for i in 0..com.len(){
    let record = format!("\n{},{},{}",com[i], min[i],zone[i]);  
    file.write_all(record.as_bytes()).unwrap();

    println!("{}",record);
}

println!("Success!");
}


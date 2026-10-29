struct Laptop{
    brand:String,
    price_each:u32
}
impl Laptop {
    fn cost_3(&self)->u32{
        self.price_each*3
    }
}
fn main() {
    println!("Welcome to Alaba Electronics! \nWe currently have 30 laptops in stock across 4 different brands \nHP----------10 units, \nIBM----------6 units, \nToshiba-----10 units, \nDell---------4 units");
    println!("If you are curious on how much i would be to buy 3 of every Laptop then ill show you.");
    let hp_3 = Laptop {
        brand:String::from("HP"),
        price_each:650_000
    };
    let ibm_3 = Laptop {
        brand:String::from("IBM"),
        price_each:755_000
    };
    let tosh_3 = Laptop {
        brand:String::from("Toshiba"),
        price_each:550_000
    };
    let dell_3 = Laptop {
        brand:String::from("Dell"),
        price_each:850_000
    };
    let final_cost = hp_3.cost_3()+ibm_3.cost_3()+tosh_3.cost_3()+dell_3.cost_3();
    let sum = hp_3.price_each+ibm_3.price_each+dell_3.price_each+dell_3.price_each;
    println!("Since \n A {} is N{} \n A {} is N{} \n A {} is N{} and \n A {} is N{}",hp_3.brand,hp_3.price_each,ibm_3.brand,ibm_3.price_each,tosh_3.brand,tosh_3.price_each,dell_3.brand,dell_3.price_each);
    println!("And the sum of all of the is N{}",sum);
    println!("When multiplied by 3 the final cost should be N{} \nYour total cost is N{}",final_cost,final_cost);


}

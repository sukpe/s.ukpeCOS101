use std::io;

fn main() {
    
    //intro to the program
    println!("Welcome to the PAU Rustacian Resturant menu!!");
    println!("Dear user from this point i would like to use your name.\nSo can you kindly input your name?");

        
        loop {
        //getting user's name
        println!("Input your name here.");
        let mut name_in = String::new();
        io::stdin()
        .read_line(&mut name_in)
        .expect("Failed to read your input. Please try again.");
        let name = name_in.trim();
            
        if name.is_empty(){
            println!("Come so You dont have name?? >:(");
            println!("I dont think your name is meant to be blank please try again.");
            continue;
        }else{
        //The Main Resturant
        let  p = "Pounded Yam & Edinkaiko Soup";
        let  f = "Fried Rice & Chicken";
        let  a = "Amala & Ewedu Soup";
        let  e = "Eba & Egusi Soup";
        let  w = "White Rice & Stew";
        let  i = "Invalid!!";
        println!("Welcome {} this is a rust powered resturant menu!!",name);
        println!("We have a large variety of foods to choose from:\n(P){}      3,200 Naira\n(F){}              3,000 Naira\n(A){}                2,500 Naira\n(E){}                  2,000 Naira\n(W){}                 2,500 Naira",p,f,a,e,w);
        
        //Loop to recieve my users order.
        println!("{},Kindly input the alphabet that is found in the bracket of what you would like to order",name);
        let buy = loop{
        let mut order_in = String::new();
        io::stdin()
        .read_line(&mut order_in)
        .expect("Failed to read your input. Please try again.");
        let order = order_in.trim().to_lowercase();
    
         let stock = match order.as_str() {
            "p" =>   p.trim(),
            "f" =>   f.trim(),
            "a" =>   a.trim(),
            "e" =>   e.trim(),
            "w" =>   w.trim(),
            _=>      i.trim(),
        };
            if stock != i{
                println!("{}",stock);
                break stock;
            } else {
                println!("{}",stock);
                println!("Ahh!!, Ahh! Cheeky boy!! XD\n{},Please ensure that your input is one of the shortlisted alphabets in the menu",name);
                continue;
            };
            
        };
        
        //How many packs are they buying and calculating new price
            println!("We sell our food in pack(s) so the prices above is for one take-out plate.\n{},How many take out plate would you like to buy?",name );
            let units :u8 = loop{ 
                let mut units_in = String::new();
                io::stdin()
                .read_line(&mut units_in)
                .expect("Failed to read your input. Please try again.");
                
                match units_in.trim().parse(){
                    Ok(num) => if num > 0 {break num;}else {println!("Please input a value greater than 0.");},
                    Err(_) =>  {println!("{},Your input is not a valid value or it is too large.\nPlease try again.",name );continue;},
                    };
                };
                println!("{} pack(s) of {}",units,buy);
                
                //calculation of total cost
                let cost:u32 = match buy{
                    "Pounded Yam & Edinkaiko Soup" => 3200,
                    "Fried Rice & Chicken" => 3000,
                    "Amala & Ewedu Soup" => 2500,
                    "Eba & Egusi Soup" => 2000,
                    "White Rice & Stew" => 2500,
                    _=>0,
                };
                let last_price:u32 = cost *( units as u32 );
                println!("{},Everything will be {} Naira",name,last_price); 

                //Asking if the user wants to make another order
                println!("Thank you for choosing us {}.\nWould you like to make another purchase?",name);
                println!("Input your choice down below either: Y(for Yes) or N (for No)");
                let check = loop {
                    let mut choice_in = String::new();
                    io::stdin()
                    .read_line(&mut choice_in)
                    .expect("Failed to read your input. Please try again.");

                    let choice = match choice_in.trim().to_lowercase().as_str(){
                        "y" => 1,
                        "n" => 0,
                        _ =>   2,
                    };
                    if choice < 2 { 
                            println!("Alright {} your choice has been taken!!",name);
                            break  choice;
                        } else {
                            println!("Invalid input. Please try again.");
                        };
                };
                
                //use of if to determine my users next course of acion
                if check == 0 {
                    println!("{},You ordered {} pack(s) of {}\nThat is {} Naira per pack",name,units,buy,cost);
                    println!("You Final cost is {}",last_price);
                    println!("{},Thank you once again for choosing us please come back soon!!",name);
                    
                } else {
                    println!("{},You ordered {} pack(s) of {}\nThat is {} Naira per pack",name,units,buy,cost);
                    println!("You Final cost is {}",last_price);
                    println!("Kindly take note!: {},you can only only order a maximum of 3 times at a time and you have currently used ONE therefore you have TWO more purchases to go!",name);
                    
                    //The Main Resturant_2
        let  p_2 = "Pounded Yam & Edinkaiko Soup";
        let  f_2 = "Fried Rice & Chicken";
        let  a_2 = "Amala & Ewedu Soup";
        let  e_2 = "Eba & Egusi Soup";
        let  w_2 = "White Rice & Stew";
        let  i_2 = "Invalid!!";
        println!("Welcome once again {} like before this is a rust powered resturant menu!!",name);
        println!("We have a large variety of foods to choose from:\n(P){}      3,200 Naira\n(F){}              3,000 Naira\n(A){}                2,500 Naira\n(E){}                  2,000 Naira\n(W){}                 2,500 Naira",p,f,a,e,w);
        
        //Loop to recieve my users order._2
        println!("{},Kindly input the alphabet that is found in the bracket of what you would like to order",name);
        let buy_2 = loop{
        let mut order_in_2 = String::new();
        io::stdin()
        .read_line(&mut order_in_2)
        .expect("Failed to read your input. Please try again.");
        let order_2 = order_in_2.trim().to_lowercase();
    
         let stock_2 = match order_2.as_str() {
            "p" =>   p_2.trim(),
            "f" =>   f_2.trim(),
            "a" =>   a_2.trim(),
            "e" =>   e_2.trim(),
            "w" =>   w_2.trim(),
            _=>      i_2.trim(),
        };
            if stock_2 != i{
                println!("{}",stock_2);
                break stock_2;
            } else {
                println!("{}",stock_2);
                println!("Ahh!!, Ahh! Cheeky boy!! XD\n{},Please ensure that your input is one of the shortlisted alphabets in the menu",name);
                continue;
            };
            
        };
        
        //How many packs are they buying and calculating new price_2
            println!("We sell our food in pack(s) so the prices above is for one take-out plate.\n{},How many take out plate would you like to buy?",name );
            let units_2 :u8 = loop{ 
                let mut units_in_2 = String::new();
                io::stdin()
                .read_line(&mut units_in_2)
                .expect("Failed to read your input. Please try again.");
                
                match units_in_2.trim().parse(){
                    Ok(num_2) => if num_2 > 0 {break num_2;}else {println!("Please input a value greater the 0.");},
                    Err(_) =>  {println!("{},Your input is not a valid value or it is too large.\nPlease try again.",name );continue;},
                    };
                };
                println!("{} pack(s) of {}",units_2,buy_2);
                
                //calculation of total cost_2
                let cost_2:u32 = match buy_2{
                    "Pounded Yam & Edinkaiko Soup" => 3200,
                    "Fried Rice & Chicken" => 3000,
                    "Amala & Ewedu Soup" => 2500,
                    "Eba & Egusi Soup" => 2000,
                    "White Rice & Stew" => 2500,
                    _=>0,
                };
                let last_price_2:u32 = cost_2 *( units_2 as u32 );
                println!("{},Everything will be {} Naira",name,last_price_2); 

                //Asking if the user wants to make another order_2
                println!("Thank you for choosing us {}.\nWould you like to make another purchase?",name);
                println!("Input your choice down below either: Y(for Yes) or N (for No)");
                let check_2 = loop {
                    let mut choice_in_2 = String::new();
                    io::stdin()
                    .read_line(&mut choice_in_2)
                    .expect("Failed to read your input. Please try again.");

                    let choice_2 = match choice_in_2.trim().to_lowercase().as_str(){
                        "y" => 1,
                        "n" => 0,
                        _ =>   2,
                    };
                    if choice_2 < 2 { println!("Alright {} your choice has been taken!!",name);
                            break  choice_2;
                        } else {
                            println!("Invalid input. Please try again.");
                        };
                };
                
                //use of if to determine my users next course of acion_2 
                if check_2 == 0 {
                    let last_price_2_f = last_price + last_price_2;
                    println!("{},You ordered {} pack(s) of {} & {} pack(s) of {}\nThat is {} Naira & {} Naira respectively per pack.",name,units,buy,units_2,buy_2,cost,cost_2);
                    println!("You Final cost is {}",last_price_2_f);
                    println!("{},Thank you once again for choosing us please come back soon!!",name);
                    
                } else {
                    let last_price_2_f = last_price + last_price_2;
                    println!("{},You ordered {} pack(s) of {} & {} pack(s) of {}\nThat is {} Naira & {} Naira respectively per pack.",name,units,buy,units_2,buy_2,cost,cost_2);
                    println!("You Final cost is {}",last_price_2_f);
                    println!("Kindly take note!!: {},You can only only order a maximum of 3 times at a time and you have currently used TWO therefore you have ONE more purchases to go!",name);
                    
                    //The Main Resturant_3
        let  p_3 = "Pounded Yam & Edinkaiko Soup";
        let  f_3 = "Fried Rice & Chicken";
        let  a_3 = "Amala & Ewedu Soup";
        let  e_3 = "Eba & Egusi Soup";
        let  w_3 = "White Rice & Stew";
        let  i_3 = "Invalid!!";
        println!("Welcome once again {} like before this is a rust powered resturant menu!!",name);
        println!("We have a large variety of foods to choose from:\n(P){}      3,200 Naira\n(F){}              3,000 Naira\n(A){}                2,500 Naira\n(E){}                  2,000 Naira\n(W){}                 2,500 Naira",p,f,a,e,w);
        
        //Loop to recieve my users order._3
        println!("{},Kindly input the alphabet that is found in the bracket of what you would like to order",name);
        let buy_3 = loop{
        let mut order_in_3 = String::new();
        io::stdin()
        .read_line(&mut order_in_3)
        .expect("Failed to read your input. Please try again.");
        let order_3 = order_in_3.trim().to_lowercase();
    
         let stock_3 = match order_3.as_str() {
            "p" =>   p_3.trim(),
            "f" =>   f_3.trim(),
            "a" =>   a_3.trim(),
            "e" =>   e_3.trim(),
            "w" =>   w_3.trim(),
            _=>      i_3.trim(),
        };
            if stock_3 != i{
                println!("{}",stock_3);
                break stock_3;
            } else {
                println!("{}",stock_3);
                println!("Ahh!!, Ahh! Cheeky boy!! XD\n{},Please ensure that your input is one of the shortlisted alphabets in the menu",name);
                continue;
            };
            
        };
        
        //How many packs are they buying and calculating new price_3
            println!("We sell our food in pack(s) so the prices above is for one take-out plate.\n{},How many take out plate would you like to buy?",name );
            let units_3 :u8 = loop{ 
                let mut units_in_3 = String::new();
                io::stdin()
                .read_line(&mut units_in_3)
                .expect("Failed to read your input. Please try again.");
                
                match units_in_3.trim().parse(){
                    Ok(num_3) => if num_3 > 0 {break num_3;}else {println!("Please input a value greater the 0.");},
                    Err(_) =>  {println!("{},Your input is not a valid value or it is too large.\nPlease try again.",name );continue;},
                    };
                };
                println!("{} pack(s) of {}",units_3,buy_3);
                
                //calculation of total cost_3
                let cost_3:u32 = match buy_3{
                    "Pounded Yam & Edinkaiko Soup" => 3200,
                    "Fried Rice & Chicken" => 3000,
                    "Amala & Ewedu Soup" => 2500,
                    "Eba & Egusi Soup" => 2000,
                    "White Rice & Stew" => 2500,
                    _=>0,
                };
                let last_price_3:u32 = cost_3 *( units_3 as u32 );
                println!("{},Everything will be {} Naira",name,last_price_3); 
                
                //use of if to show their final cost.
                    println!("Kindly take note!!!: {},you can only only order a maximum of 3 times  at a time and you have currently used THREE therefore you have NO more purchases to go!",name);
                    let last_price_3_f = last_price + last_price_2 + last_price_3;
                    println!("{},You ordered {} pack(s) of {}, {} pack(s) of {}, & {} pack(s) of {}\nThat is {} Naira, {} Naira, & {} Naira respectivel per pack.",name,units,buy,units_2,buy_2,units_3,buy_3,cost,cost_2,cost_3);
                    println!("You Final cost is {}",last_price_3_f);
                    println!("Thank you once again for choosing us!\nYou are a Wonderful customer {}", name);
                }
                }
            
                };
            break;
            }           
             
        }
            

            
        
    


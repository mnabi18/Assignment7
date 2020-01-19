mod main_mod{
       pub mod sub_mod{  
            pub  fn table(inp:u32){
          println!("We are in sub module --> table function");
            for count in 1..=5 {
                println!("{} *{}={}",inp,count,inp*count);
            }
        }
    }
}
use std::io;
fn main() {
    loop {
        println!("Please Enter a Number: ");
        
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1)
        .expect("Failed to read line");
        let input1: u32 = match input1.trim().parse(){
            Ok(num) => num,
            Err(_)=> continue,
        };
        println!("You entered : {}", input1);
        
        main_mod::sub_mod::table(input1);
        
        break;
    }
}

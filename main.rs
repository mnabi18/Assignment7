mod lib;
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
     
     lib::main_mod::sub_mod::table(input1);
     
     break;
 }
}

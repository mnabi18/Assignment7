use std::io;
use libpakg;
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
    libpakg::main_mod::sub_mod::math1(input1);
    break;
}
}
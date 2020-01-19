   pub mod main_mod{
      pub  mod sub_mod{
         pub   fn table(inp:u32){
            println!("We are in sub module --> table function");
            for count in 1..=5 {
                println!("{} + {}={}",inp,count,inp+count);
            }
        }
    }
}
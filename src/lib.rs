use std::error::Error;
use std::fs;

pub fn run(config: Cfg) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename).expect("someting wong");        // my code is very neat ik
    println!("Text: \n {} ", contents);
    Ok(())
} 
pub struct Cfg { //config
    pub query: String,
    pub filename: String,


}
impl Cfg {
    pub fn new(args: &[String]) -> Result<Cfg, &str>{
        if args.len() < 3 {
            return  Err("not enough arguments buddy");
        }

        let query = args[1].clone();
        let filename = args[2].clone();   // i am being tortured
        Ok(Cfg { query , filename })
    
    }



}
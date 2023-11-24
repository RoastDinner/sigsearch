use std::env; // pp
use std::fs;
use std::process;



fn main(){

    let args: Vec<String> = env::args().collect(); // kill me  

    let config = Cfg::new(&args).unwrap_or_else(|err| {
        println!("problem with: {}",err);
        process::exit(1);
    });


    println!("searching for {}", config.query);
    println!("in file {}", config.filename);
    
    let contents = fs::read_to_string(config.filename).expect("someting wong");        // my code is very neat ik


    println!("Text: \n {} ", contents);


}

struct Cfg { //config
    query: String,
    filename: String,


}
impl Cfg {
    fn new(args: &[String]) -> Result<Cfg, &str>{
        if args.len() < 3 {
            return  Err("not enough arguments buddy");
        }

        let query = args[1].clone();
        let filename = args[2].clone();   // i am being tortured
        Ok(Cfg { query , filename })
    
    }



}


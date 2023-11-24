use std::env;
use std::process;
use sigsearch::Cfg;
fn main(){

    let args: Vec<String> = env::args().collect(); // kill me  

    let config = Cfg::new(&args).unwrap_or_else(|err| {
        println!("problem with: {}",err);
        process::exit(1);
    });


    println!("searching for {}", config.query);
    println!("in file {}", config.filename);
    
    if let Err(e) = sigsearch::run(config){
        println!("app error: {}", e);
        process::exit(1);


    }


}



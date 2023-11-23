use std::env;
use std::fs;




fn main(){

    let args: Vec<String> = env::args().collect(); // kill me  
    let query = &args[1];
    let filename = &args[2];

    println!("searching for {}", query);
    println!("in file {}", filename);
    
    let contents = fs::read_to_string(filename).expect("someting wong");        // my code is very neat ik


    println!("Text: \n {} ", contents);
}
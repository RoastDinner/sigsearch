use std::error::Error;
use std::fs;

// go away

pub fn run(config: Cfg) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename).expect("someting wong");        // my code is very neat ik
   for line in search(&config.query, &contents){

    println!("{}", line);


   }
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
pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oneresult(){
        let query = "tes";
        let content = "test, tes, te, p";
        assert_eq!(vec!["test, tes, te, p"], search(query,content));


    }


}
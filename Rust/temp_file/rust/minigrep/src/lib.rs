
use std::fs;
use std::process;

pub struct Config{
    pub text:String,
    pub filename:String
 }
impl Config {
    pub fn new(args:Vec<String>)->Result<Config,&'static str>{
        match args.len() {
            3=>{
                let text=args[1].clone();
                let filename=args[2].clone();
                Ok(Config{text,filename})
            },
            _=>Err("Enter only 2 args! : one for search text and one for filepath")
        }
    }
 }
pub fn run(config:Result<Config,&str>) {
     match config {
         Ok(config)=>{
             let content=fs::read_to_string(config.filename).expect("There's some error in path");
             println!("{}",content);
             let res=search(config.text.as_str(),content.as_str());

             println!("The query: {:?} is found in lines: \n{:#?}",config.text,res);
         },
         Err(err)=>{
             println!("{}",err);
             process::exit(1);
         }
     }
    
 }

pub fn search<'a>(query:&str, content:&'a str)->Vec<&'a str>{
// vec![]
let mut lines_including_query=content.lines().filter(|line| line.contains(&query.to_lowercase())).collect();
lines_including_query
}

 #[cfg(test)]
 mod tests {
    use super::*;
    #[test]
    fn one_res(){
        let query="duct";
        let content="\
Rust
Safe, Fast, Productive";
        assert_eq!(vec!["Safe, Fast, Productive"], search(query,content));
    }    
 }

//  "https://doc.rust-lang.org/book/ch06-00-enums.html#enums-and-pattern-matching
// https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html
// https://www.youtube.com/watch?v=DSZqIJhkNCM 
// https://www.youtube.com/watch?v=NzeQbiiqhgo"

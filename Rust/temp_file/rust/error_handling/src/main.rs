use std::{error, io};
use std::fs::File;
// use std::io::stdin;
use std::io::{ErrorKind, Read, Write};
fn main() {
    let f=File::open("hello.txt");    
    
    let mut f=match f{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=>match File::create("hello.txt"){
                Ok(fs)=>fs,
                Err(error)=>panic!("error: {}",error)
            },
            
            other_err=>{
                panic!("problem opening file {}",other_err)
            }

        }
    };
    // let file=File::open("abc.txt").unwrap_or_else(|err| {
    //     if err.kind()==ErrorKind::NotFound{
    //         File::create("abc.txt").unwrap_or_else(|err|{
    //             panic!("err");
    //         })
    //     }else {
    //         panic!("Problem in opening file {}",err);
    //     }
    // });
    let mut s=String::new();
    match f.read_to_string(&mut s){
        Ok(_)=>{println!("{}",s)},
        Err(error)=>{println!("Error : {}",error)}
            
    }

    // match f.write(b"Bye") {
    //     Ok(_)=>(),
    //     Err(error)=>println!("Error is: {}",error)}
    // f.write_all("abc".as_bytes()).expect("Unable to write data");

}

fn read_username_from_file()->Result<String,io::Error>{
    let mut str=String::new();
    File::open("hello.txt")?.read_to_string(&mut str)?;
    Ok(str)
}

// use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess the number!\n");
    let secret_num=rand::thread_rng().gen_range(0..100);

    println!("The randomly generated number is: {}",secret_num);
    
    loop {
        println!("Please input the guess: ");

        let mut guess=String::with_capacity(100);
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to print line");
        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{
                println!("Enter a num my man!");
                continue;
            },
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_num){
            Ordering::Less=>println!("{}","Too small!".red()),
            Ordering::Equal=>{
                println!("{}","You win".green());
                break;  
            },
            Ordering::Greater=>println!("{}","Too great!".blue()),
            // _=>println!("Enter a num my man")
        }    
    }

}


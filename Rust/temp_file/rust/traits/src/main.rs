use std::{iter::Sum, fmt::Display, result};

struct Tweet{
    username:String,
    content:String,
    reply:bool,
    retweet:bool
}
impl Summary for Tweet {
    fn summarize(&self)->String {
        format!("{}:{}",self.username,self.content)
    }
    fn content(&self)->String {
        format!("{}",self.content)
    }
}


struct NewsArticle{
    author:String,
    headline:String,
    content:String
}

impl Summary for NewsArticle {
    fn summarize(&self)->String {
        format!("{}, by {}", self.headline,self.author)
    }
    fn content(&self)->String {
        format!("{}...Read More",self.content)
    }
}

trait Summary {
    fn summarize(&self)->String{
        "Default Summary".to_string()
    }
    fn content(&self)->String{
        "Default content".to_string()
    }
}
trait  Read {
    
}

pub fn notify<T:Summary>(item:&T) {
    println!("Breaking news! Read {}", item.summarize());
}

fn returns_summarizable()-> impl Summary{ //This function will return any type of data that
    //implements the summary trait. vert useful in closures.
    Tweet{
        username:"books".to_string(),
        content:"abc".to_string(),
        reply:false,
        retweet:false
    }
}

struct Pair<T>{
    x:T,
    y:T
}

impl <T>Pair<T> { //This will be implemented for all type
    fn new(x:T,y:T)->Self{
        Self{x,y}
    }
}

impl <T:PartialOrd+Display>Pair<T> { //Only for pair struct types that implement trait PartialOrder and Display like i, u, vec of i, u ,float etc. 
    fn cmp_display() {
        
    }
}

impl <T:Summary> Read for T { //Implementing the Read trait for any
    //type  THAT IMPLEMENTS THE SUMMARY TRAIT
    //Widely used in Std lib
    
}

fn longest<'a>(x:&'a str, y:& str)-> &'a str{
    // if x.len()>y.len(){
    //     x
    // }
    // else {
    //     y    
    // }
        x
}


fn announcement<'a,T:Display>(
    x:&'a str,
    y:&'a str,
    ann:T
)->&'a str
// where
//     T:Display
    {
        println!("{}",ann);
        if x.len()>y.len(){
            x
        }
        else{
            y
        }
    }
fn takes_float(float:f64){
    {
        let y="abc".to_string();
    }
    println!("{}",y);
}
//Lifetime
fn main() {
    let tweet=Tweet{
        username:"John Doe".to_string(),
        content:"Hello World".to_string(),
        reply:false,
        retweet:false
    };
    println!("{}",tweet.summarize());
    println!("{}",tweet.content());
    

    let article=NewsArticle{
        author:"John Doe".to_string(),
        headline:"Hello World".to_string(),
        content:"Good Morning".to_string()
    };
    println!("{}",article.summarize());
    println!("{}",article.content());
    notify(&article);

    let x=String::from("Hello");
    let result;
    {
        let y=String::from("HelloHello");
        result=longest(&x, &y);
    }

    println!("{}",result);
    // let float:f64=;
    // takes_float(float);
    // println!("{}",float);
    {
        let x="hello".to_string();
    }
    println!("{}",x);
}

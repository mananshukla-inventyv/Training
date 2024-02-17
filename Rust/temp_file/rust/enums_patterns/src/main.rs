mod ret_str;
fn main() {
    coin_state(Coin::Quarter(States::Us));

    let number=77;
    match number{
        1=>{println!("ONE")}
        2|3|5|7=>{println!("Prime")}
        13..=19=>{println!("Teen")}
        _=>()
    }
    let static_s=ret_str::ret_str();
    println!("{}",static_s);
    let mut new_str="hellooooooooo".to_string();
    let slice_str=&mut new_str[1..3].to_string();
    slice_str.push_str("manan");
    println!("{:?}",slice_str);
    println!("{:?}",new_str);
}


#[derive(Debug)]
enum States{
    Us,
    Canada(String)
}
impl States {
    fn somee(){
        println!("abc");
    }
}

enum Coin {
    Penny,
    Dime,
    Quarter(States)
}

fn coin_state(coin:Coin)->u8 {
    match coin {
        Coin::Penny=>{
            println!("Penny!");
            1
        }
        Coin::Dime=>{2}
        Coin::Quarter(state)=>{
            println!("Quarter works in {:?}", state);
            3
        }
    }
}
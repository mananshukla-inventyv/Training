use core::num;
use std::string;

fn main() {
    let x=10;
    // const y:i32=x*x;  cant use const to store this or any other compile time computations.
    let tup=("hello",123);
    let (name,id)=tup;
    let finalName=tup.0;
    let finalId=tup.1;

    // Arrays
    let my_arr=[1,2,3];
    // let val=my_arr[1];
    println!("{:#?}",my_arr); //pretty print of whole array
    println!("{:#?}",my_arr);// normal print of whole 
    // println!(my_arr[1]);
    let byte=[0;8];
    println!("{}",byte[0]);
    let sum=sum(10, 20);
    println!("{}",sum);

    //if else in rust- always needs a boolean val
    if sum<10 {
        println!("too small")
    }
    else if sum==10 {
        println!("perfect")
    }
    else {
        println!("too big")
    }

    //like python
    let cond=true;
    let final_cond=if cond {5} else {6};
    println!("{}",final_cond);

    println!("Types of loops in rust");
    let counter=looppp();
    println!("{}",counter);

    //ownership in rust:
    // 1. Each value has a variable that is its owner
    // 2. Each val needs to have one owner
    // 3. After the value goes out of score or re-assignment(acctualy copy) then the ownership is changed
    
    let s1=String::from("hello");
    let s2=s1;

    // println!("{}", s1); borrow of moved val error
    println!("{}", s2);
    // int bools and char are copied not moved

    let mut str1=String::from("Hello Ref");
    // let mut num1=23;

    let r1=&mut str1;
    // let n1=&mut num1;
    // let n2=&mut num1;
    r1.push_str(" Pushedd"); //can only push when we have mutable val and mutable ref passed
    // let r2=& str1;
    // println!("{} , {}", r1,r2);
    println!("{} r1 here",r1);
    let r2=& str1; // here this can be used after the scope of the reference ends
    //scope of the reference ends where its last used!
    // so basically here the ref r1 ends when its used above in println
    // println!("{} , {}", n1,n2); same for numbers
    println!("{} r2 here",r2);

    //DANGLING REFERENCES

    // let s=dangle(); WIll throw error as the the string reference its referring to will get destroyed and the ref will point to invalid memory

    let byte_char: u8 = b' ';
    println!("Byte char: {}", byte_char);

    // b" " is a byte string literal
    let byte_str: &[u8] = b" ";
    println!("Byte string: {:#?}", byte_str);
    let mut s=String::from("Hello World");
    let word=first_word(&s);
    // s.clear();
    // println!("{}", word);

}

fn looppp()->i32 {
    let mut counter=0;
    loop {
        counter+=1;
        println!("again");
        
        if counter==10 {
            break counter;
    }
    
    let mut number=0;
    while number!=0 {
        println!("{}",number);
        number-=1;
    }
    let a=[1,23,4,5,6];
    for element in a {
        println!("{}",element);
    }
}
}




fn sum(x:i32,y:i32)-> i32 {
    println!("The val of x is: {}",x);
    println!("The val of y is: {}",y);
    x+y

}

// fn dangle()->&String {
//     let s=String::from("Hello");
//     &s
// }

fn first_word(s:&String)->&str {
    let b=s.as_bytes();
    println!("{:?}", b);
    for (i,&item) in b.iter().enumerate(){
        if item==b' '{
           return  &s[..i];
        }
        
    }
    &s[..]
    
}
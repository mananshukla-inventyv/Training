use std::{vec, collections::{HashMap, hash_map}};
use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug)]
enum SpreadSheets {
    Text(String),
    Int(i32),
    Bool(bool)
}
fn main() {
 let mut first_vec=vec!["1".to_string(),"2".to_string(),"3".to_string()];
 first_vec.push("3".to_string());
 println!("{:?}",first_vec);
 println!("{}",first_vec[0]);   
 mut_vec(&mut first_vec);
 let mut str="abc".to_string();
 str.push_str("a");

 let multi_val_vec=vec![
    SpreadSheets::Text("abc".to_string()),
    SpreadSheets::Int(12),
    SpreadSheets::Bool(true)
 ];
 match &multi_val_vec[0] {
     SpreadSheets::Text(s)=>{println!("{}",s)},
    //  SpreadSheets::Bool(b)=>(),
    //  SpreadSheets::Int(i)=>()
    _=>()
 };
// println!("{:?}",multi_val_vec[0]);
    let hello = "Здравствуйте";
    let my_arr=[1,2,3,4];
    let ma=&my_arr[0..2]; //Here the & pointer here is a fat pointer and this helps in referencing dynamically sized types
    // let ma=my_arr[..2];
    // Because it is unsized, [T] cannot implement 
    // IntoIterator, which is the trait used for
    //  iterating over things in for loops.
    let s = &hello[0..4];
    println!("{}",s);   
    for i in hello.graphemes(true){
        println!("{}",i);
    }
    let blue="blue".to_string();

    let mut hm=HashMap::new();
    hm.insert(&blue, 1);
    // println!("{}",blue)
    let get_val="bluee".to_string();
    let val=hm.get(&get_val);
    println!("{}",val.unwrap_or(&10));

    let text="Hello world abc world";

    let mut hm=HashMap::new();
    for i in text.split_whitespace(){
        let count=hm.entry(i).or_insert(0);
        *count+=1;
        // This line increments the count of the current word by 1. 
        // The count variable holds a mutable
        //  reference to the value associated with the current word in the HashMap.
        
    }
    println!("{:?}",hm);

}
fn mut_vec(vector:&mut Vec<String>){
    vector.push("value".to_string());
    println!("{:?}",vector);
    vector[0].push_str("one");
    println!("{:?}",vector);

}
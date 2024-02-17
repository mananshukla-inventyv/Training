use employee::employee;
use serde::{Deserialize,Serialize};
// use serde_json::Value;
use std::fs;
pub mod employee;
pub mod test;
#[derive(Serialize,Deserialize, Debug)]
struct Employee{
    name:String,
    age:i32,
    skills:Vec<String>,
    position:Option<String>,
    #[serde(rename = "experiance(y)")]
    experience:Option<i32>
}

fn main() {
    // let json_data=serde_json::to_string()
    employee();

}
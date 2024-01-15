use serde::{Deserialize,Serialize};
use serde_json::Value;
use std::fs;

#[derive(Serialize,Deserialize, Debug, Clone)]
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
    let final_data=fs::read_to_string("Employee.json");
    match final_data{
        Ok(data)=>{
            let final_data:Result<Vec<Employee>,serde_json::Error>=serde_json::from_str(&data);
            println!("{:#?}",final_data);
            
            match final_data {
                Ok(data)=>{
                    let mut mid_and_rust:Vec<Employee>=vec![];
                    let mut jr_and_java:Vec<Employee>=vec![];
                    let mut sr_or_c:Vec<Employee>=vec![];
                    for i in 0..data.len(){
                      if data[i].position==Some("Software Developer".to_string()) && data[i].skills.contains(&"Rust".to_string()){
                        mid_and_rust.push(data[i].clone());
                      }
                      if data[i].position==Some("Software Developer".to_string()) && data[i].skills.contains(&"Java".to_string()){
                        jr_and_java.push(data[i].clone());
                      }
                      if data[i].position==Some("Sr. Software Developer".to_string()) || data[i].skills.contains(&"C#".to_string()){
                        sr_or_c.push(data[i].clone());
                      }
                    }

                    fs::write("mid_and_rust.json", serde_json::to_string_pretty(&mid_and_rust).expect("msg")).expect("msg");
                    fs::write("mid_and_java.json", serde_json::to_string_pretty(&jr_and_java).expect("msg")).expect("msg");
                    fs::write("sr_or_c.json", serde_json::to_string_pretty(&sr_or_c).expect("msg")).expect("msg");

                }

                _=>()
            }

        
        }
        
        Err(_)=>()
    };

}

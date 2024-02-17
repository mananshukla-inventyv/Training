use serde::{Deserialize,Serialize};
// use serde_json::Value;
use std::fs;


#[derive(Serialize,Deserialize, Debug)]

enum Position {
    #[serde(rename = "Software Developer")]
    SoftwareDeveloper,
    #[serde(rename = "Sr. Software Developer")]
    SeniorDeveloper,
    #[serde(rename = "Jr. Software Developer")]
    JuniorDeveloper, 
    #[serde(rename = "Project Manager")]
    ProjectManager,
    #[serde(rename = "Team Lead")]
    TeamLead

}

#[derive(Serialize,Deserialize, Debug,PartialEq,Default)]
enum Skills {
    #[serde(rename = "Rust")]
    #[default] Rust,
    #[serde(rename = "Java")]
    Java,
    #[serde(rename = "C#")]
    CSharp, 
    #[serde(rename = "Python")]
    Python

}
#[derive(Serialize,Deserialize, Debug,Default)]
struct Employee{
    name:String,
    age:i32,
    skills:Vec<Skills>,
    position:Option<Position>,
    #[serde(rename = "experiance(y)")]
    experience:Option<i32>
}

fn main() {
    // let json_data=serde_json::to_string()
    let w:Result<Skills,f32>=Err(0.0);
        let res=w.unwrap_or_default();
        println!("ans is : {:?}",res);  
    
    let final_data=fs::read_to_string("Employee.json");
    match final_data{
        Ok(data)=>{
            let final_data:Result<Vec<Employee>,serde_json::Error>=serde_json::from_str(&data);
            // println!("{:#?}",final_data);
            
            match final_data {
                Ok(data)=>{
                    let mut mid_and_rust:Vec<Employee>=vec![];
                    let mut jr_and_java:Vec<Employee>=vec![];
                    let mut sr_or_c:Vec<Employee>=vec![];
                    for i in data{

                        match i.position {
                            Some(Position::SoftwareDeveloper)=>{
                                if i.skills.contains(&Skills::Rust){
                                    mid_and_rust.push(i)
                                } 
                                
                            }
                            Some(Position::JuniorDeveloper)=>{
                                if i.skills.contains(&Skills::Java){
                                    jr_and_java.push(i)
                                }
                            }
                            Some(Position::SeniorDeveloper)=>{
                                if i.skills.contains(&Skills::CSharp){
                                    sr_or_c.push(i);
                                }
                            }
                            _=>{
                                if i.skills.contains(&Skills::CSharp){
                                    sr_or_c.push(i)
                                }
                            }
                            
                            
                        }
                    }
                    fs::write("mid_and_rust.json", serde_json::to_string_pretty(&mid_and_rust).expect("msg")).expect("msg");
                    fs::write("jr_and_java.json", serde_json::to_string_pretty(&jr_and_java).expect("msg")).expect("msg");
                    fs::write("sr_or_c.json", serde_json::to_string_pretty(&sr_or_c).expect("msg")).expect("msg");

                }

                _=>()
            }  
        }
        
        Err(_)=>()
    };

}
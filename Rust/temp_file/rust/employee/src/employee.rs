use std::fs;

use crate::Employee;

pub fn employee()->(usize,usize,usize){
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
                    for i in data{
                      if i.position==Some("Software Developer".to_string()) && i.skills.contains(&"Rust".to_string()){
                        mid_and_rust.push(i);
                      }
                      else if i.position==Some("Jr. Software Developer".to_string()) && i.skills.contains(&"Java".to_string()){
                        jr_and_java.push(i);
                      }
                      else if i.position==Some("Sr. Software Developer".to_string()) || i.skills.contains(&"C#".to_string()){
                        sr_or_c.push(i);
                      }
                      else{
                        ()
                      }
                    }

                    fs::write("mid_and_rust.json", serde_json::to_string_pretty(&mid_and_rust).expect("msg")).expect("msg");
                    fs::write("jr_and_java.json", serde_json::to_string_pretty(&jr_and_java).expect("msg")).expect("msg");
                    fs::write("sr_or_c.json", serde_json::to_string_pretty(&sr_or_c).expect("msg")).expect("msg");
                    (mid_and_rust.len(),jr_and_java.len(),sr_or_c.len())
                }

                _=>(
                    (0,0,0)
                )
            }
        
        }
        Err(err)=>{
            (0,0,0)
        }
    }
}
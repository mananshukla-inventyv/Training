use std::{collections::HashMap, fs};
use super::type_collections::{Employee, Position, Skills};
use serde_json::{Value,json};
/// Processes employee data from a JSON file, categorizes employees based on position and skills,
/// and writes the categorized data to separate JSON files.
pub fn employee_hashmap_task() {
    // Read employee data from the JSON file
    let final_data = fs::read_to_string("./data/Employee.json");

    match final_data {
        Ok(data) => {
            // Parse the JSON data into a vector of Employee structs
            let final_data: Result<Vec<Employee>, serde_json::Error> = serde_json::from_str(&data);

            match final_data {
                Ok(data) => {
                    // Create vectors to store categorized employees
                    let mut mid_and_rust = vec![];
                    let mut jr_and_java = vec![];
                    let mut sr_or_c = vec![];
                    
 
                    // Iterate through each employee and categorize based on position and skills
                    for employee in data{
                        let mut each_emp_hmap=HashMap::new();
                        each_emp_hmap.insert("name", Value::String(employee.name));
                        each_emp_hmap.insert("age", json!(employee.age));
                        each_emp_hmap.insert("skills", json!(employee.skills));
                        each_emp_hmap.insert("position", json!(employee.position));
                        each_emp_hmap.insert("experiance(y)", json!(employee.experience));


                        match employee.position {
                            Some(Position::SoftwareDeveloper) => {
                                if employee.skills.contains(&Skills::Rust) {
                                    mid_and_rust.push(each_emp_hmap);
                                }
                            }
                            Some(Position::JuniorDeveloper) => {
                                if employee.skills.contains(&Skills::Java) {
                                    jr_and_java.push(each_emp_hmap);
                                }   
                            }
                            Some(Position::SeniorDeveloper) | None => {
                                if employee.skills.contains(&Skills::CSharp) {
                                    sr_or_c.push(each_emp_hmap);
                                }
                            }
                            _=>{
                                if employee.skills.contains(&Skills::CSharp) {
                                    sr_or_c.push(each_emp_hmap);
                                }
                            }
                        }
                    }
                    let mut filtered_emp_hashmap=HashMap::new();
                    filtered_emp_hashmap.insert("mid_and_rust", mid_and_rust);
                    filtered_emp_hashmap.insert("jr_and_java", jr_and_java);
                    filtered_emp_hashmap.insert("sr_or_c", sr_or_c);
                    fs::write("./data/filtered_emp.json", serde_json::to_string_pretty(&filtered_emp_hashmap).expect("msg")).expect("msg");
                }
                Err(error) => {
                    // Handle potential errors during JSON parsing
                    eprintln!("Error parsing JSON data: {}", error);
                }
            }
        }
        Err(error) => {
            // Handle potential errors during file reading
            eprintln!("Error reading employee data file: {}", error);
        }
    };
}
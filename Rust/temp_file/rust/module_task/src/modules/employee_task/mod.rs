use std::fs;
use super::type_collections::{Employee, Position, Skills};

/// Processes employee data from a JSON file, categorizes employees based on position and skills,
/// and writes the categorized data to separate JSON files.
pub fn employee_task() {
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
                    for employee in data {
                        match employee.position {
                            Some(Position::SoftwareDeveloper) => {
                                if employee.skills.contains(&Skills::Rust) {
                                    mid_and_rust.push(employee);
                                }
                            }
                            Some(Position::JuniorDeveloper) => {
                                if employee.skills.contains(&Skills::Java) {
                                    jr_and_java.push(employee);
                                }
                            }
                            Some(Position::SeniorDeveloper) | None => {
                                if employee.skills.contains(&Skills::CSharp) {
                                    sr_or_c.push(employee);
                                }
                            }
                            _=>{
                                if employee.skills.contains(&Skills::CSharp) {
                                    sr_or_c.push(employee);
                                }
                            }
                        }
                    }

                    // Write categorized data to separate JSON files
                    fs::write("./data/mid_and_rust.json", serde_json::to_string_pretty(&mid_and_rust).expect("msg")).expect("msg");
                    fs::write("./data/jr_and_java.json", serde_json::to_string_pretty(&jr_and_java).expect("msg")).expect("msg");
                    fs::write("./data/sr_or_c.json", serde_json::to_string_pretty(&sr_or_c).expect("msg")).expect("msg");
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


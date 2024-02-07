//! De-serializes the coming JSON data and then calculate the grades and percent based on the field markk, updates the data and then serializes it again

use crate::modules::type_collections::Student;
use std::fs;

/// Processes student data from a JSON file, calculates percentages and grades,
/// and updates the data in a new JSON file.
pub fn student_task() {
    // Read student data from the JSON file
    let stud_data = fs::read_to_string("./data/StudentData.json").expect("Wrong Path");

    // Deserialize the JSON data into a vector of Student objects
    let mut data: Vec<Student> = serde_json::from_str(&stud_data).expect("err");

    // Iterate through each student, calculate percentage and grade, and update the data
    for i in 0..data.len() {
        data[i].calc_percent_grades();
    }

    // Serialize the updated data back to JSON and write it to a new file
    let sr_data = serde_json::to_string_pretty(&data).unwrap();
    fs::write("./data/final_data.json", &sr_data).expect("msg");
}

//! De-serializes the coming JSON data and then calculate the grades and percent based on the field markk, updates the data and then serializes it again

use std::{collections::HashMap, fs};
use crate::modules::type_collections::Student;
use serde_json::{Value,json};

/// Converts a vector of Student structs to a vector of HashMaps with string keys and serde_json Values.
///
/// This function takes a vector of Student structs and transforms each struct into a HashMap,
/// where each field of the Student struct is represented as a key-value pair in the HashMap.
/// The keys are strings, and the values are serde_json Values. The resulting vector of HashMaps
/// can be useful for serialization or further processing.
///
/// # Arguments
///
/// * `vec` - A vector of Student structs to be converted.
///
/// # Returns
///
/// Returns a vector of HashMaps, where each HashMap represents a Student with field names as keys
/// and serde_json Values as corresponding values.

pub fn vec_to_hashmap(vec:Vec<Student>)->Vec<HashMap<&'static str, Value>> {
    let mut student_vec=Vec::new();
    for i in vec {
        let mut each_stud_hmap=HashMap::new();
        each_stud_hmap.insert("name", Value::String(i.name));
        each_stud_hmap.insert("phone", Value::String(i.phone));
        each_stud_hmap.insert("email", Value::String(i.email));
        each_stud_hmap.insert("city", Value::String(i.city));
        each_stud_hmap.insert("address", Value::String(i.address));
        each_stud_hmap.insert("marks", json!(i.marks));
        each_stud_hmap.insert("percent", json!(i.percent));
        each_stud_hmap.insert("grade", Value::String(i.grade.unwrap_or_default()));
        student_vec.push(each_stud_hmap);
        
    }
    student_vec
}


/// Processes student data from a JSON file, calculates percentages and grades,
/// and updates the data in a new JSON file.

pub fn student_hashmap_task() {
    // Read student data from the JSON file
    let stud_data = fs::read_to_string("./data/StudentData.json").expect("Wrong Path");

    // Deserialize the JSON data into a vector of Student objects
    let mut data: Vec<Student> = serde_json::from_str(&stud_data).expect("err");
    for i in 0..data.len() {data[i].calc_percent_grades();}
    
    
    // Iterate through each student, calculate percentage and grade, and update the data
    let student_vec=vec_to_hashmap(data);

    // Serialize the updated data back to JSON and write it to a new file
    let sr_data = serde_json::to_string_pretty(&student_vec).unwrap();
    fs::write("./data/student_hmap_data.json", &sr_data).expect("msg");
}

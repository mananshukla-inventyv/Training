//! This module stores all the needed enum and struct types later used in the program.

use serde::{Deserialize, Serialize};

/// Enum representing different positions in a company.
#[derive(Serialize, Deserialize, Debug)]
pub enum Position {
    #[serde(rename = "Software Developer")]
    SoftwareDeveloper,
    #[serde(rename = "Sr. Software Developer")]
    SeniorDeveloper,
    #[serde(rename = "Jr. Software Developer")]
    JuniorDeveloper,
    #[serde(rename = "Project Manager")]
    ProjectManager,
    #[serde(rename = "Team Lead")]
    TeamLead,
}

/// Enum representing different skills an employee can have.
#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub enum Skills {
    #[serde(rename = "Rust")]
    #[default]
    Rust,
    #[serde(rename = "Java")]
    Java,
    #[serde(rename = "C#")]
    CSharp,
    #[serde(rename = "Python")]
    Python,
}

/// Struct representing an employee in a company.
#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    /// The name of the employee.
    pub name: String,
    /// The age of the employee.
    pub age: i32,
    /// The skills possessed by the employee.
    pub skills: Vec<Skills>,
    /// The position held by the employee.
    pub position: Option<Position>,
    /// The years of experience of the employee.
    #[serde(rename = "experiance(y)")]
    pub experience: Option<i32>,
}

/// Struct representing a student.
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Student {
    /// The name of the student.
    pub name: String,
    /// The phone number of the student.
    pub phone: String,
    /// The email address of the student.
    pub email: String,
    /// The city where the student resides.
    pub city: String,
    /// The address of the student.
    pub address: String,
    /// The marks obtained by the student in various subjects.
    pub marks: Vec<u16>,
    /// The percentage obtained by the student.
    pub percent: Option<f32>,
    /// The grade assigned to the student.
    pub grade: Option<String>,
}

// Implementation of a method for the Student type
impl Student {
    // Method to calculate the percentage and grade for a student
    pub fn calc_percent_grades(&mut self) {
        // Calculate the total marks using the sum of the marks in the student's marks vector
        let total: u16 = self.marks.iter().sum();
        
        // Calculate the percentage based on the total marks and the number of subjects
        let percent = (total as f32) / self.marks.len() as f32;
        
        // Determine the grade based on the calculated percentage
        let grade = match percent as u16 {
            0..=20 => {"E".to_string()},
            21..=40 => {"D".to_string()},
            41..=60 => {"C".to_string()},
            61..=80 => {"B".to_string()},
            81..=100 => {"A".to_string()},
            _ => { panic!("invalid marks!") }
        };

        self.percent=Some(percent);
        self.grade=Some(grade);
        
        // Return a tuple containing the percentage and grade
        // (percent, grade) 
    }
}

/// Struct representing the response of a task involving character analysis.
#[derive(Debug)]
pub struct Task2Response {
    /// Vector containing characters and their counts.
    pub letter_count_vec: Vec<(char, u8)>,
    /// Vector containing characters left out after analysis.
    pub left_out: Vec<(char, u8)>,
}

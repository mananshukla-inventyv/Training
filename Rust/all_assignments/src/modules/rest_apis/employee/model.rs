use serde::{Deserialize, Serialize};


/// Enum representing different positions in a company.
#[derive(Serialize, Deserialize, Debug,Clone)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Default,Clone)]
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
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Employee {
    pub id: String,
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

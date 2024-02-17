use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Message<T> {
    pub status: u32,
    pub message_key: String,
    pub data: T,
}

#[derive(Serialize, Debug, Deserialize,Clone)]
pub struct Student {
    pub id: u64,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
}



#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Employee {
    pub id:u64,
    pub name: String,
    pub age: u16,
    pub skills: Vec<Skills>,
    pub position: Option<Position>,
    #[serde(rename="experiance(y)")]
    pub experiance: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum Skills {
    #[serde(rename = "Java")]
    Java,
    #[serde(rename = "C#")]
    Csharp,
    #[serde(rename = "Rust")]
    Rust,
    #[serde(rename = "Python")]
    Python,
}

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum Position {
    #[serde(rename = "Sr. Software Developer")]
    SrSoftwareDeveloper,
    #[serde(rename = "Jr. Software Developer")]
    JrSoftwareDeveloper,
    #[serde(rename = "Software Developer")]
    SoftwarDeveloper,
    #[serde(rename = "Team Lead")]
    TeamLead,
    #[serde(rename = "Project Manager")]
    ProjectManager,
}




#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub skills: Vec<Skill>,
    pub status: Status,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum Language {
    English,
    Spanish,
}

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum Status {
    Online,
    Offline,
}

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum Skill {
    #[serde(rename = "Customer Service")]
    Customer_Service,
    #[serde(rename = "Problem-solving")]
    Problem_solving,
    #[serde(rename = "Product Knowledge")]
    Product_Knowledge,
    #[serde(rename = "Effective Communication")]
    Effective_Communication,
    #[serde(rename = "Time Management")]
    Time_Management,
    #[serde(rename = "Adaptability")]
    Adaptability,
    #[serde(rename = "Team Collaboration")]
    Team_Collaboration,
    #[serde(rename = "Feedback Analysis")]
    Feedback_Analysis,
    #[serde(rename = "Proactive Engagement")]
    Proactive_Engagement,
    #[serde(rename = "Technical Proficiency")]
    Technical_Proficiency,
    #[serde(rename = "Cultural Sensitivity")]
    Cultural_Sensitivity,
    #[serde(rename = "Documentation")]
    Documentation,
}


lazy_static! {
    #[derive(Clone,Debug)]
    pub static ref STUDENT: Arc<RwLock<HashMap<u64, Student>>> =
        Arc::new(RwLock::new(HashMap::new()));

    #[derive(Clone,Debug)]
    pub static ref EMPLOYEE: Arc<RwLock<HashMap<u64, Employee>>> =
        Arc::new(RwLock::new(HashMap::new()));

    #[derive(Clone,Debug)]
    pub static ref USER: Arc<RwLock<HashMap<u64, User>>> =
        Arc::new(RwLock::new(HashMap::new()));
}

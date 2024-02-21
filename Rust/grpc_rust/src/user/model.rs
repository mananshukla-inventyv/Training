use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub skills: Vec<String>,
    pub status: StatusUser,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum StatusUser {
    #[serde(rename = "Online")]
    Online,
    #[serde(rename = "Offline")]
    Offline,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Language {
    #[serde(rename = "English")]
    English,
    #[serde(rename = "Spanish")]
    Spanish,
}

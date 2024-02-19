//! This module stores all the needed enum and struct types later used in the program.

use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub skills: Vec<String>,
    pub status: Status,
    pub language: Language,
}

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum Status {
    Online,
    Offline,
}

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub enum Language {
    English,
    Spanish,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Req {
    pub task_type: SupportType,
    pub skills: String,
    pub language: Language,
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SupportType {
    OnCall,
    OnChat,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Message<T>{
    pub status:i32,
    pub message_key:String,
    pub data:T
}




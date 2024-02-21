use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
pub struct Student {
    pub id: i32,
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
    pub marks: Vec<u32>,
    /// The percentage obtained by the student.
    pub percent: Option<f32>,
    /// The grade assigned to the student.
    pub grade: Option<String>,
}

use serde::{Deserialize, Serialize};

/// Struct representing a student.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Student {
    pub id: String,
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


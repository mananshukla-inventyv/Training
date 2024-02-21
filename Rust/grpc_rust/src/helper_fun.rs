use crate::{
    employee::model::{Employee, Position, Skills},
    student::model::Student,
    user::model::{Language, StatusUser, User},
};

use self::ser::EmployeeData as emp;
use self::ser::StudentData as stdd;
use self::ser::UserData;
pub mod ser {
    tonic::include_proto!("services");
}

pub fn create_user_struct(user: UserData) -> User {
    User {
        id: user.id,
        name: user.name,
        skills: user.skills,
        status: string_to_status(user.status), // No position for User struct
        language: string_to_language(user.language), // No experience for User struct
    }
}

pub fn create_user_msg(user: User) -> UserData {
    UserData {
        id: user.id,
        name: user.name,
        skills: user.skills,
        status: status_to_string(user.status), // No position for User struct
        language: language_to_string(user.language),
    }
}

pub fn string_to_status(status: String) -> StatusUser {
    match status.as_str() {
        "Online" => StatusUser::Online,
        "Offline" => StatusUser::Offline,
        _ => StatusUser::Offline, // Default to Offline if status is unrecognized
    }
}

pub fn status_to_string(status: StatusUser) -> String {
    match status {
        StatusUser::Online => String::from("Online"),
        StatusUser::Offline => String::from("Offline"),
    }
}

pub fn string_to_language(language: String) -> Language {
    match language.as_str() {
        "English" => Language::English,
        "Spanish" => Language::Spanish,
        _ => Language::English, // Default to English if language is unrecognized
    }
}

pub fn language_to_string(language: Language) -> String {
    match language {
        Language::English => String::from("English"),
        Language::Spanish => String::from("Spanish"),
    }
}

pub fn create_student_struct(stud: stdd) -> Student {
    Student {
        id: stud.id,
        name: stud.name,
        phone: stud.phone,
        email: stud.email,
        city: stud.city,
        address: stud.address,
        marks: stud.marks,
        percent: stud.percent,
        grade: stud.grade,
    }
}

pub fn create_student_msg(stud: Student) -> stdd {
    stdd {
        id: stud.id,
        name: stud.name,
        phone: stud.phone,
        email: stud.email,
        city: stud.city,
        address: stud.address,
        marks: stud.marks,
        percent: stud.percent,
        grade: stud.grade,
    }
}

pub fn create_employee_struct(employee: emp) -> Employee {
    Employee {
        id: employee.id,
        name: employee.name,
        age: employee.age,
        skills: skill_to_enum(employee.skills),
        position: position_to_enum(employee.position),
        experience: employee.experience,
    }
}

pub fn create_employee_msg(employee: Employee) -> emp {
    emp {
        id: employee.id,
        name: employee.name,
        age: employee.age,
        skills: enum_to_skill(employee.skills),
        position: enum_to_position(employee.position),
        experience: employee.experience,
    }
}

pub fn enum_to_skill(skills: Vec<Skills>) -> Vec<String> {
    let mut skill_vec = vec![];
    for skill in skills {
        let skill_str = match skill {
            Skills::CSharp => "C#".to_string(),
            Skills::Java => "Java".to_string(),
            Skills::Python => "Python".to_string(),
            Skills::Rust => "Rust".to_string(),
        };
        skill_vec.push(skill_str);
    }
    skill_vec
}

pub fn enum_to_position(position: Option<Position>) -> Option<String> {
    if !position.is_none() {
        let pos_str = match position.unwrap() {
            Position::JuniorDeveloper => "Jr. Software Developer".to_string(),
            Position::SeniorDeveloper => "Sr. Software Developer".to_string(),
            Position::SoftwareDeveloper => "Software Developer".to_string(),
            Position::ProjectManager => "Project Manager".to_string(),
            Position::TeamLead => "Team Lead".to_string(),
        };
        Some(pos_str)
    } else {
        None
    }
}

pub fn position_to_enum(position: Option<String>) -> Option<Position> {
    if !position.is_none() {
        let pos_str = match position.unwrap().as_str() {
            "Jr. Software Developer" => Position::JuniorDeveloper,
            "Sr. Software Developer" => Position::SeniorDeveloper,
            "Software Developer" => Position::SoftwareDeveloper,
            "Project Manager" => Position::ProjectManager,
            "Team Lead" => Position::TeamLead,
            _ => Position::SoftwareDeveloper,
        };
        Some(pos_str)
    } else {
        None
    }
}

pub fn skill_to_enum(skills: Vec<String>) -> Vec<Skills> {
    let mut skill_vec = vec![];
    for skill in skills {
        let skill_str = match skill.as_str() {
            "C#" => Skills::CSharp,
            "Java" => Skills::Java,
            "Python" => Skills::Python,
            "Rust" => Skills::Rust,
            _ => Skills::Rust,
        };
        skill_vec.push(skill_str);
    }
    skill_vec
}

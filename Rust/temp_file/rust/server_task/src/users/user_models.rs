// //! This module stores all the needed enum and struct types later used in the program.

// use std::{
//     collections::HashMap,
//     fs,
//     sync::{Arc, RwLock},
// };

// use lazy_static::lazy_static;
// use serde::{Deserialize, Serialize};

// lazy_static! {
//     static ref STUDENT_DATA: Arc<RwLock<HashMap<String, Student>>> =
//         Arc::new(RwLock::new(HashMap::new()));
//     static ref EMPLOYEE_DATA: Arc<RwLock<HashMap<String, Employee>>> =
//         Arc::new(RwLock::new(HashMap::new()));
//     static ref USER_DATA: Arc<RwLock<HashMap<String, User>>> =
//         Arc::new(RwLock::new(HashMap::new()));
// }

// /// Enum representing different positions in a company.
// #[derive(Serialize, Deserialize, Debug,Clone)]
// pub enum Position {
//     #[serde(rename = "Software Developer")]
//     SoftwareDeveloper,
//     #[serde(rename = "Sr. Software Developer")]
//     SeniorDeveloper,
//     #[serde(rename = "Jr. Software Developer")]
//     JuniorDeveloper,
//     #[serde(rename = "Project Manager")]
//     ProjectManager,
//     #[serde(rename = "Team Lead")]
//     TeamLead,
// }

// /// Enum representing different skills an employee can have.
// #[derive(Serialize, Deserialize, Debug, PartialEq, Default,Clone)]
// pub enum Skills {
//     #[serde(rename = "Rust")]
//     #[default]
//     Rust,
//     #[serde(rename = "Java")]
//     Java,
//     #[serde(rename = "C#")]
//     CSharp,
//     #[serde(rename = "Python")]
//     Python,
// }

// /// Struct representing an employee in a company.
// #[derive(Serialize, Deserialize, Debug,Clone)]
// pub struct Employee {
//     pub id: String,
//     /// The name of the employee.
//     pub name: String,
//     /// The age of the employee.
//     pub age: i32,
//     /// The skills possessed by the employee.
//     pub skills: Vec<Skills>,
//     /// The position held by the employee.
//     pub position: Option<Position>,
//     /// The years of experience of the employee.
//     #[serde(rename = "experiance(y)")]
//     pub experience: Option<i32>,
// }

// /// Struct representing a student.
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Student {
//     pub id: String,
//     /// The name of the student.
//     pub name: String,
//     /// The phone number of the student.
//     pub phone: String,
//     /// The email address of the student.
//     pub email: String,
//     /// The city where the student resides.
//     pub city: String,
//     /// The address of the student.
//     pub address: String,
//     /// The marks obtained by the student in various subjects.
//     pub marks: Vec<u16>,
//     /// The percentage obtained by the student.
//     pub percent: Option<f32>,
//     /// The grade assigned to the student.
//     pub grade: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
// pub struct User {
//     pub id: String,
//     pub name: String,
//     pub skills: Vec<String>,
//     pub status: Status,
//     pub language: Language,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
// pub enum Status {
//     Online,
//     Offline,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq,Clone)]
// pub enum Language {
//     English,
//     Spanish,
// }
// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub struct Req {
//     pub task_type: SupportType,
//     pub skills: String,
//     pub language: Language,
// }
// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub enum SupportType {
//     OnCall,
//     OnChat,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub struct Message<T>{
//     pub status:i32,
//     pub message_key:String,
//     pub data:T
// }

// use axum::{extract::Path, http::Request, response::{IntoResponse, Response}, Json};

// // pub async fn get_item(Path(var): Path<(String, i32)>) -> String {
// //     format!("got the {} with id: {}", var.0, var.1)
// // }
// // pub async fn create_item(Path(route_type): Path<String>) -> String {
// //     format!("created the {}", route_type)
// // }
// // pub async fn delete_item(Path(var): Path<(String, i32)>) -> String {
// //     format!("deleted the {} with id: {}", var.0, var.1)
// // }
// // pub async fn update_item_student(Path(id): Path<i32>,Json(stud):Json<Student>)->Response {
// //     let mut data_to_be_updated=STUDENT_DATA.write().unwrap();
// //     if stud.id==id.to_string(){    
// //         let student_to_be_updated=data_to_be_updated.get_mut(&id.to_string()).unwrap();
// //         student_to_be_updated.name=stud.name;
// //         student_to_be_updated.email=stud.email;
// //         Json(
// //             Message{
// //                 status:2000,
// //                 message_key:"Users Updated".to_string(),
// //                 data:Some(student_to_be_updated.clone()) 
// //             }
// //         ).into_response()
// //     }
// //     else {
// //         Json(
// //             Message{
// //                 status:2001,
// //                 message_key:"Users Not Found".to_string(),
// //                 data:"".to_string()
// //             }
// //         ).into_response()
// //     }

// //     // format!("Updated {} woth id : {} check by using get commnad",var.0,var.1)
// // }
// // pub async fn update_item_emp(Path(var): Path<i32>,Json(emp): Json<Employee>) -> Response {
// //     let mut data_to_be_updated=EMPLOYEE_DATA.write().unwrap();
// //     if emp.id==var.to_string(){    
// //         let emp_to_be_updated=data_to_be_updated.get_mut(&var.to_string()).unwrap();
// //         emp_to_be_updated.name=emp.name;
// //         // student_to_be_updated.email=emp.email;
// //         Json(
// //             Message{
// //                 status:2000,
// //                 message_key:"Users Updated".to_string(),
// //                 data:emp_to_be_updated.clone() 
// //             }
// //         ).into_response()
// //     }
// //     else {
// //         Json(
// //             Message{
// //                 status:2001,
// //                 message_key:"Users Not Found".to_string(),
// //                 data:""
// //             }
// //         ).into_response()
// //     }
// // }
// // pub async fn update_item_user(Path(var): Path<i32>,Json(user):Json<User>) -> Response {
// //     let mut data_to_be_updated=USER_DATA.write().unwrap();
// //     if let Some(user_to_be_updated)=data_to_be_updated.get_mut(&var.to_string()){
// //         user_to_be_updated.name=user.name;
// //         user_to_be_updated.id=user.id;
// //         user_to_be_updated.skills=vec!["a".to_string(),"b".to_string(),"c".to_string()];
// //         user_to_be_updated.status=Status::Online;
// //         user_to_be_updated.language=Language::English;
// //         Json(
// //             Message{
// //                 status:2000,
// //                 message_key:"Users Updated".to_string(),
// //                 data:user_to_be_updated.clone() 
// //             }
// //         ).into_response()
// //     }
// //     else {
// //         Json(
// //             Message{
// //                 status:2001,
// //                 message_key:"Users Not Found".to_string(),
// //                 data:""
// //             }
// //         ).into_response()
// //     }
// // }

// // pub async fn get_all(Path(var): Path<String>) -> Response {
// //     if var=="user".to_string(){
// //         Json(
// //             Message{
// //                 status:3000,
// //                 message_key:"Fetched all users".to_string(),
// //                 data:
// //                     USER_DATA.read().unwrap().clone()
// //             }
// //         ).into_response()
// //     }
// //     else if var=="student".to_string() {
// //         Json(
// //             Message{
// //                 status:3000,
// //                 message_key:"Fetched all users".to_string(),
// //                 data:
// //                     STUDENT_DATA.read().unwrap().clone()
// //             }
// //         ).into_response()
// //     }
// //     else {
// //         Json(
// //             Message{
// //                 status:3000,
// //                 message_key:"Fetched all users".to_string(),
// //                 data:
// //                     EMPLOYEE_DATA.read().unwrap().clone()
// //             }
// //         ).into_response()
// //     }
// // }


// // pub async fn data_loader() {
// //     let std_vec: Vec<Student> =
// //         serde_json::from_str(&fs::read_to_string("./data/StudentData.json").unwrap()).unwrap();
// //     let user_vec: Vec<User> =
// //         serde_json::from_str(&fs::read_to_string("./data/Master_Data.json").unwrap()).unwrap();
// //     let emp_vec: Vec<Employee> =
// //         serde_json::from_str(&fs::read_to_string("./data/Employee.json").unwrap()).unwrap();

// //     for each_std in std_vec {
// //         STUDENT_DATA
// //             .write()
// //             .unwrap()
// //             .insert(each_std.id.clone(), each_std);
// //     }
// //     for each_user in user_vec {
// //         USER_DATA
// //             .write()
// //             .unwrap()
// //             .insert(each_user.id.clone(), each_user);
// //     }
// //     for each_emp in emp_vec {
// //         EMPLOYEE_DATA
// //             .write()
// //             .unwrap()
// //             .insert(each_emp.id.clone(), each_emp);
// //     }
// // }

// pub async fn add_user(Json(payload): Json<User>) -> Response {
//     if USER_DATA.read().unwrap().contains_key(&payload.id) {
//         return {
//             Json(Message {
//                 status: 2000,
//                 message_key: String::from("success"),
//                 data: format!("User id {} already exist", payload.id),
//             })
//         }
//         .into_response();
//     }
    
//     match USER_DATA.write() {
//         Ok(mut data) => {
//             data.insert(payload.id.clone(), payload);
//             return {
//                 Json(Message {
//                     status: 2000,
//                     message_key: String::from("success"),
//                     data: format!("User add successfully"),
//                 })
//             }
//             .into_response();
//         }
//         Err(_) => {
//             return {
//                 Json(Message {
//                     status: 4000,
//                     message_key: String::from("fail"),
//                     data: format!("fail to add User in file"),
//                 })
//             }
//             .into_response();
//         }
//     }
// }

// pub async fn update_user(Json(payload): Json<User>) -> Response {
//     if USER_DATA.read().unwrap().contains_key(&payload.id) {
//         match USER_DATA.write() {
//             Ok(mut data) => {
//                 data.entry(payload.id.clone()).or_insert(payload);
//                 return {
//                     Json(Message {
//                         status: 2000,
//                         message_key: String::from("success"),
//                         data: format!("User update successfully"),
//                     })
//                 }
//                 .into_response();
//             }
//             Err(_) => {
//                 return {
//                     Json(Message {
//                         status: 4000,
//                         message_key: String::from("fail"),
//                         data: format!("fail to update User in file"),
//                     })
//                 }
//                 .into_response();
//             }
//         }
//     } else {
//         return {
//             Json(Message {
//                 status: 4000,
//                 message_key: String::from("fail"),
//                 data: format!("User id {} not exist", payload.id),
//             })
//         }
//         .into_response();
//     }
// }

// pub async fn delete_user(Json(payload): Json<serde_json::Value>) -> Response {
//     let id: u64 = payload.get("id").and_then(|id| id.as_u64()).unwrap();
//     if USER_DATA.read().unwrap().contains_key(&id) {
//         match USER_DATA.write() {
//             Ok(mut data) => {
//                 data.remove(&id);
//                 return {
//                     Json(Message {
//                         status: 2000,
//                         message_key: String::from("success"),
//                         data: format!("User delete successfully"),
//                     })
//                 }
//                 .into_response();
//             }
//             Err(_) => {
//                 return {
//                     Json(Message {
//                         status: 4000,
//                         message_key: String::from("fail"),
//                         data: format!("fail to delete User in file"),
//                     })
//                 }
//                 .into_response();
//             }
//         }
//     } else {
//         return {
//             Json(Message {
//                 status: 4000,
//                 message_key: String::from("fail"),
//                 data: format!("User id {} not exist", id),
//             })
//         }
//         .into_response();
//     }
// }

// pub async fn get_all_user() -> Response {
//     return {
//         Json(Message {
//             status: 2000,
//             message_key: String::from("success"),
//             data: USER_DATA.read().unwrap().clone(),
//         })
//     }
//     .into_response();
// }

// pub async fn get_user(Path(id): Path<u64>) ->Response{
//      if USER_DATA.read().unwrap().contains_key(&id) {
//         return {
//             Json(Message {
//                 status: 2000,
//                 message_key: String::from("success"),
//                 data: USER_DATA.read().unwrap().get(&id).unwrap().clone()
//             })
//         }
//         .into_response();
//      }
//      else{
//         return {
//             Json(Message {
//                 status: 4000,
//                 message_key: String::from("fail"),
//                 data: format!("User id {} not exist", id),
//             })
//         }
//         .into_response();
//      }
// }

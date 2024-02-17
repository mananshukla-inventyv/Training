pub mod common;
pub mod config;
pub mod health_check;
pub mod index;
pub mod service;
pub mod middlewares;

use std::{fs, net::SocketAddr};

use self::{
    common::{Employee, Student, User, EMPLOYEE, STUDENT, USER},
    index::get_routes, middlewares::get_middlewares,
};

#[tokio::main]
pub async fn main_file() {
    match fs::read_to_string("src/service/operation/axum/StudentData.json") {
        Ok(mut student_str) => {
            let mut student_data: Result<Vec<Student>, serde_json::Error> =
                serde_json::from_str(&student_str);
            match student_data {
                Ok(mut students) => {
                    let mut i: u64 = 1;
                    for student in students {
                        STUDENT.write().unwrap().insert(i, student);
                        i += 1;
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    match fs::read_to_string("src/service/operation/axum/Employee.json") {
        Ok(mut employee_str) => {
            let mut employee_data: Result<Vec<Employee>, serde_json::Error> =
                serde_json::from_str(&employee_str);
            match employee_data {
                Ok(mut employees) => {
                    let mut i: u64 = 1;
                    for employee in employees {
                        EMPLOYEE.write().unwrap().insert(i, employee);
                        i += 1;
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    match fs::read_to_string("src/service/operation/axum/Master_Data.json") {
        Ok(mut user_str) => {
            let mut user_data: Result<Vec<User>, serde_json::Error> =
                serde_json::from_str(&user_str);
            match user_data {
                Ok(mut users) => {
                    let mut i: u64 = 1;
                    for user in users {
                        USER.write().unwrap().insert(i, user);
                        i += 1;
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
    // println!("{:#?}", STUDENT.read().unwrap());
    let app = get_routes();
    let app = get_middlewares(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // let _server = axum::Server::bind(&addr)
    //     .serve(app.clone().into_make_service())
    //     .await
    //     .expect("something went wrong!");
}

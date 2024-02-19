use std::{collections::HashMap, fs, sync::{Arc, RwLock}};

use employee::model::Employee;
use lazy_static::lazy_static;
use routes::create_routes;
use student::model::Student;
use tokio::net::TcpListener;
use users::model::User;

pub mod config;
pub mod health_check;
pub mod helper_func;
pub mod middleware;
pub mod routes;
pub mod users;
pub mod student;
pub mod employee;

lazy_static! {
    static ref STUDENT_DATA: Arc<RwLock<HashMap<String, Student>>> =
        Arc::new(RwLock::new(HashMap::new()));
    static ref EMPLOYEE_DATA: Arc<RwLock<HashMap<String, Employee>>> =
        Arc::new(RwLock::new(HashMap::new()));
    static ref USER_DATA: Arc<RwLock<HashMap<String, User>>> =
        Arc::new(RwLock::new(HashMap::new()));
}


pub async fn data_loader() {
    let std_vec: Vec<Student> =
        serde_json::from_str(&fs::read_to_string("./data/StudentData_Server.json").unwrap()).unwrap();
    let user_vec: Vec<User> =
        serde_json::from_str(&fs::read_to_string("./data/Master_Data_Server.json").unwrap()).unwrap();
    let emp_vec: Vec<Employee> =
        serde_json::from_str(&fs::read_to_string("./data/Employee_Server.json").unwrap()).unwrap();

    for each_std in std_vec {
        STUDENT_DATA
            .write()
            .unwrap()
            .insert(each_std.id.clone(), each_std);
    }
    for each_user in user_vec {
        USER_DATA
            .write()
            .unwrap()
            .insert(each_user.id.clone(), each_user);
    }
    for each_emp in emp_vec {
        EMPLOYEE_DATA
            .write()
            .unwrap()
            .insert(each_emp.id.clone(), each_emp);
    }
}



pub async fn run() {
    let app = create_routes().await;
    // let addr=SocketAddr::new([((0,0,0,0),3000)]);
    let listener = TcpListener::bind(&"0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

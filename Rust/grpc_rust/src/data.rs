use std::{collections::HashMap, fs, sync::{Arc, RwLock}};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{employee::model::Employee, student::model::Student, user::model::User};


lazy_static! {
    #[derive(Debug,Serialize,Deserialize)]
    pub static ref STUDENT_DATA: Arc<RwLock<HashMap<String, Student>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref EMPLOYEE_DATA: Arc<RwLock<HashMap<String, Employee>>> =
        Arc::new(RwLock::new(HashMap::new()));
    pub static ref USER_DATA: Arc<RwLock<HashMap<String, User>>> =
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
            .insert(each_std.id.to_string().clone(), each_std);
    }
    for each_user in user_vec {
        USER_DATA
            .write()
            .unwrap()
            .insert(each_user.id.to_string().clone(), each_user);
    }
    for each_emp in emp_vec {
        EMPLOYEE_DATA
            .write()
            .unwrap()
            .insert(each_emp.id.to_string().clone(), each_emp);
    }
}


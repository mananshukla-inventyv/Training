use axum::{extract::Path, response::{IntoResponse, Response}, Json};


use crate::modules::rest_apis::{users::model::Message, EMPLOYEE_DATA};

use super::model::Employee;


pub async fn update_item_emp(Path(var): Path<i32>,Json(emp): Json<Employee>) -> Response {
    let mut data_to_be_updated=EMPLOYEE_DATA.write().unwrap();
    if emp.id==var.to_string(){    
        let emp_to_be_updated=data_to_be_updated.get_mut(&var.to_string()).unwrap();
        emp_to_be_updated.name=emp.name;
        // student_to_be_updated.email=emp.email;
        Json(
            Message{
                status:2000,
                message_key:"Users Updated".to_string(),
                data:emp_to_be_updated.clone() 
            }
        ).into_response()
    }
    else {
        Json(
            Message{
                status:2001,
                message_key:"Users Not Found".to_string(),
                data:""
            }
        ).into_response()
    }
}

pub async fn delete_emp(Path(var): Path<i32>)-> Response{
    let mut data_to_be_deleted=EMPLOYEE_DATA.write().unwrap();
    if let Some(user_to_be_deleted)=data_to_be_deleted.remove(&var.to_string()){
        Json(
            Message{
                status:2000,
                message_key:"User deleted".to_string(),
                data:user_to_be_deleted.clone() 
            }
        ).into_response()
    }
    else {
        Json(
            Message{
                status:2001,
                message_key:"Users Not Found".to_string(),
                data:""
            }
        ).into_response()
    }
}

use axum::{extract::Path, response::{IntoResponse, Response}, Json};

use crate::modules::server_task::{users::model::Message, EMPLOYEE_DATA, STUDENT_DATA, USER_DATA};


pub async fn get_all(Path(var): Path<String>) -> Response {
    if var=="user".to_string(){
        Json(
            Message{
                status:3000,
                message_key:"Fetched all users".to_string(),
                data:
                    USER_DATA.read().unwrap().clone()
            }
        ).into_response()
    }
    else if var=="student".to_string() {
        Json(
            Message{
                status:3000,
                message_key:"Fetched all users".to_string(),
                data:
                    STUDENT_DATA.read().unwrap().clone()
            }
        ).into_response()
    }
    else {
        Json(
            Message{
                status:3000,
                message_key:"Fetched all users".to_string(),
                data:EMPLOYEE_DATA.read().unwrap().clone()
            }
        ).into_response()
    }
}

pub async fn get_item(Path(var): Path<String>) -> Response {
    if var=="user".to_string(){
        if let Some(user_to_be_displayed)=USER_DATA.write().unwrap().get(&var){
        Json(
            Message{
                status:3000,
                message_key:"Got the user".to_string(),
                data:user_to_be_displayed.clone()
                    
            }
        ).into_response()
    }
    else {
        Json(
            Message{
                status:3001,
                message_key:"Not Found".to_string(),
                data:""
                    
            }
        ).into_response()
    }
    }
    else if var=="student".to_string() {
        if let Some(std_to_be_displayed)=STUDENT_DATA.write().unwrap().get(&var){
            Json(
                Message{
                    status:3000,
                    message_key:"Got the item".to_string(),
                    data:std_to_be_displayed.clone()
                        
                }
            ).into_response()
        }
        else {
            Json(
                Message{
                    status:3001,
                    message_key:"Not Found".to_string(),
                    data:""
                        
                }
            ).into_response()
        }
    }
    else {
        if let Some(emp_to_be_displayed)=EMPLOYEE_DATA.write().unwrap().get(&var){
            Json(
                Message{
                    status:3000,
                    message_key:"Got the item".to_string(),
                    data:emp_to_be_displayed.clone()
                        
                }
            ).into_response()
        }
        else {
            Json(
                Message{
                    status:3001,
                    message_key:"Not Found".to_string(),
                    data:""
                        
                }
            ).into_response()
        }
    }
}
use axum::{extract::Path, response::{IntoResponse, Response}, Json};


use crate::modules::server_task::{users::model::Message, STUDENT_DATA};

use super::model::Student;

pub async fn update_item_student(Path(id): Path<i32>,Json(stud):Json<Student>)->Response {
    let mut data_to_be_updated=STUDENT_DATA.write().unwrap();
    if stud.id==id.to_string(){    
        let student_to_be_updated=data_to_be_updated.get_mut(&id.to_string()).unwrap();
        student_to_be_updated.name=stud.name;
        student_to_be_updated.email=stud.email;
        Json(
            Message{
                status:2000,
                message_key:"Users Updated".to_string(),
                data:Some(student_to_be_updated.clone()) 
            }
        ).into_response()
    }
    else {
        Json(
            Message{
                status:2001,
                message_key:"Users Not Found".to_string(),
                data:"".to_string()
            }
        ).into_response()
    }

    // format!("Updated {} woth id : {} check by using get commnad",var.0,var.1)
}

pub async fn delete_student(Path(var): Path<i32>)-> Response{
    let mut data_to_be_deleted=STUDENT_DATA.write().unwrap();
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

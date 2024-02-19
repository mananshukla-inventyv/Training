use axum::{extract::Path, response::{IntoResponse, Response}, Json};


use crate::modules::server_task::USER_DATA;

use super::model::{Language, Message, Status, User};


pub async fn update_item_user(Path(var): Path<i32>,Json(user):Json<User>) -> Response {
    let mut data_to_be_updated=USER_DATA.write().unwrap();
    if let Some(user_to_be_updated)=data_to_be_updated.get_mut(&var.to_string()){
        user_to_be_updated.name=user.name;
        user_to_be_updated.id=user.id;
        user_to_be_updated.skills=vec!["a".to_string(),"b".to_string(),"c".to_string()];
        user_to_be_updated.status=Status::Online;
        user_to_be_updated.language=Language::English;
        Json(
            Message{
                status:2000,
                message_key:"Users Updated".to_string(),
                data:user_to_be_updated.clone() 
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

pub async fn delete_users(Path(var): Path<i32>)-> Response{
    let mut data_to_be_deleted=USER_DATA.write().unwrap();
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

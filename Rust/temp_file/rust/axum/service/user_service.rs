use axum::{
    extract::Path, response::{IntoResponse, Response}, Json
};

use super::super::common::{Message, User , USER};

pub async fn add_user(Json(payload): Json<User>) -> Response {
    if USER.read().unwrap().contains_key(&payload.id) {
        return {
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: format!("User id {} already exist", payload.id),
            })
        }
        .into_response();
    }
    
    match USER.write() {
        Ok(mut data) => {
            data.insert(payload.id, payload);
            return {
                Json(Message {
                    status: 2000,
                    message_key: String::from("success"),
                    data: format!("User add successfully"),
                })
            }
            .into_response();
        }
        Err(_) => {
            return {
                Json(Message {
                    status: 4000,
                    message_key: String::from("fail"),
                    data: format!("fail to add User in file"),
                })
            }
            .into_response();
        }
    }
}

pub async fn update_user(Json(payload): Json<User>) -> Response {
    if USER.read().unwrap().contains_key(&payload.id) {
        match USER.write() {
            Ok(mut data) => {
                data.entry(payload.id).or_insert(payload);
                return {
                    Json(Message {
                        status: 2000,
                        message_key: String::from("success"),
                        data: format!("User update successfully"),
                    })
                }
                .into_response();
            }
            Err(_) => {
                return {
                    Json(Message {
                        status: 4000,
                        message_key: String::from("fail"),
                        data: format!("fail to update User in file"),
                    })
                }
                .into_response();
            }
        }
    } else {
        return {
            Json(Message {
                status: 4000,
                message_key: String::from("fail"),
                data: format!("User id {} not exist", payload.id),
            })
        }
        .into_response();
    }
}

pub async fn delete_user(Json(payload): Json<serde_json::Value>) -> Response {
    let id: u64 = payload.get("id").and_then(|id| id.as_u64()).unwrap();
    if USER.read().unwrap().contains_key(&id) {
        match USER.write() {
            Ok(mut data) => {
                data.remove(&id);
                return {
                    Json(Message {
                        status: 2000,
                        message_key: String::from("success"),
                        data: format!("User delete successfully"),
                    })
                }
                .into_response();
            }
            Err(_) => {
                return {
                    Json(Message {
                        status: 4000,
                        message_key: String::from("fail"),
                        data: format!("fail to delete User in file"),
                    })
                }
                .into_response();
            }
        }
    } else {
        return {
            Json(Message {
                status: 4000,
                message_key: String::from("fail"),
                data: format!("User id {} not exist", id),
            })
        }
        .into_response();
    }
}

pub async fn get_all_user() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: USER.read().unwrap().clone(),
        })
    }
    .into_response();
}

pub async fn get_user(Path(id): Path<u64>) ->Response{
     if USER.read().unwrap().contains_key(&id) {
        return {
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: USER.read().unwrap().get(&id).unwrap().clone()
            })
        }
        .into_response();
     }
     else{
        return {
            Json(Message {
                status: 4000,
                message_key: String::from("fail"),
                data: format!("User id {} not exist", id),
            })
        }
        .into_response();
     }
}

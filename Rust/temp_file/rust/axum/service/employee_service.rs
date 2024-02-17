use axum::{
    extract::Path, response::{IntoResponse, Response}, Json
};

use super::super::common::{Message, Employee , EMPLOYEE};

pub async fn add_employee(Json(payload): Json<Employee>) -> Response {
    if EMPLOYEE.read().unwrap().contains_key(&payload.id) {
        return {
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: format!("Employee id {} already exist", payload.id),
            })
        }
        .into_response();
    }
    
    match EMPLOYEE.write() {
        Ok(mut data) => {
            data.insert(payload.id, payload);
            return {
                Json(Message {
                    status: 2000,
                    message_key: String::from("success"),
                    data: format!("Employee add successfully"),
                })
            }
            .into_response();
        }
        Err(_) => {
            return {
                Json(Message {
                    status: 4000,
                    message_key: String::from("fail"),
                    data: format!("fail to add Employee in file"),
                })
            }
            .into_response();
        }
    }
}

pub async fn update_employee(Json(payload): Json<Employee>) -> Response {
    if EMPLOYEE.read().unwrap().contains_key(&payload.id) {
        match EMPLOYEE.write() {
            Ok(mut data) => {
                data.entry(payload.id).or_insert(payload);
                return {
                    Json(Message {
                        status: 2000,
                        message_key: String::from("success"),
                        data: format!("Employee update successfully"),
                    })
                }
                .into_response();
            }
            Err(_) => {
                return {
                    Json(Message {
                        status: 4000,
                        message_key: String::from("fail"),
                        data: format!("fail to update Employee in file"),
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
                data: format!("Employee id {} not exist", payload.id),
            })
        }
        .into_response();
    }
}

pub async fn delete_employee(Json(payload): Json<serde_json::Value>) -> Response {
    let id: u64 = payload.get("id").and_then(|id| id.as_u64()).unwrap();
    if EMPLOYEE.read().unwrap().contains_key(&id) {
        match EMPLOYEE.write() {
            Ok(mut data) => {
                data.remove(&id);
                return {
                    Json(Message {
                        status: 2000,
                        message_key: String::from("success"),
                        data: format!("Employee delete successfully"),
                    })
                }
                .into_response();
            }
            Err(_) => {
                return {
                    Json(Message {
                        status: 4000,
                        message_key: String::from("fail"),
                        data: format!("fail to delete Employee in file"),
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
                data: format!("Employee id {} not exist", id),
            })
        }
        .into_response();
    }
}

pub async fn get_all_employee() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: EMPLOYEE.read().unwrap().clone(),
        })
    }
    .into_response();
}

pub async fn get_employee(Path(id): Path<u64>) ->Response{
     if EMPLOYEE.read().unwrap().contains_key(&id) {
        return {
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: EMPLOYEE.read().unwrap().get(&id).unwrap().clone()
            })
        }
        .into_response();
     }
     else{
        return {
            Json(Message {
                status: 4000,
                message_key: String::from("fail"),
                data: format!("Employee id {} not exist", id),
            })
        }
        .into_response();
     }
}

use axum::{
    extract::Path, response::{IntoResponse, Response}, Json
};

use super::super::common::{Message, Student, STUDENT};

pub async fn add_student(Json(payload): Json<Student>) -> Response {
    if STUDENT.read().unwrap().contains_key(&payload.id) {
        return {
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: format!("Student id {} already exist", payload.id),
            })
        }
        .into_response();
    }
    // println!("{:?}",STUDENT.read().unwrap());
    match STUDENT.write() {
        Ok(mut data) => {
            data.insert(payload.id, payload);
            return {
                Json(Message {
                    status: 2000,
                    message_key: String::from("success"),
                    data: format!("Student add successfully"),
                })
            }
            .into_response();
        }
        Err(_) => {
            return {
                Json(Message {
                    status: 4000,
                    message_key: String::from("fail"),
                    data: format!("fail to add Student in file"),
                })
            }
            .into_response();
        }
    }
}

pub async fn update_student(Json(payload): Json<Student>) -> Response {
    if STUDENT.read().unwrap().contains_key(&payload.id) {
        match STUDENT.write() {
            Ok(mut data) => {
                data.entry(payload.id).or_insert(payload);
                return {
                    Json(Message {
                        status: 2000,
                        message_key: String::from("success"),
                        data: format!("Student update successfully"),
                    })
                }
                .into_response();
            }
            Err(_) => {
                return {
                    Json(Message {
                        status: 4000,
                        message_key: String::from("fail"),
                        data: format!("fail to update Student in file"),
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
                data: format!("Student id {} not exist", payload.id),
            })
        }
        .into_response();
    }
}

pub async fn delete_student(Json(payload): Json<serde_json::Value>) -> Response {
    let id: u64 = payload.get("id").and_then(|id| id.as_u64()).unwrap();
    if STUDENT.read().unwrap().contains_key(&id) {
        match STUDENT.write() {
            Ok(mut data) => {
                data.remove(&id);
                return {
                    Json(Message {
                        status: 2000,
                        message_key: String::from("success"),
                        data: format!("Student delete successfully"),
                    })
                }
                .into_response();
            }
            Err(_) => {
                return {
                    Json(Message {
                        status: 4000,
                        message_key: String::from("fail"),
                        data: format!("fail to delete Student in file"),
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
                data: format!("Student id {} not exist", id),
            })
        }
        .into_response();
    }
}

pub async fn get_all_student() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: STUDENT.read().unwrap().clone(),
        })
    }
    .into_response();
}

pub async fn get_student(Path(id): Path<u64>) ->Response{
     if STUDENT.read().unwrap().contains_key(&id) {
        return {
            Json(Message {
                status: 2000,
                message_key: String::from("success"),
                data: STUDENT.read().unwrap().get(&id).unwrap().clone()
            })
        }
        .into_response();
     }
     else{
        return {
            Json(Message {
                status: 4000,
                message_key: String::from("fail"),
                data: format!("Student id {} not exist", id),
            })
        }
        .into_response();
     }
}

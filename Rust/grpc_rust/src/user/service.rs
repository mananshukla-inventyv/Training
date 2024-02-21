use crate::{
    data::USER_DATA,
    helper_fun::{
        create_user_struct,
        ser::{user_service_server::UserService, Empty, Id, Response as resp, UserData},
        string_to_language, string_to_status,
    },
};

use tonic::{Request, Response, Result, Status};

#[derive(Debug, Default)]
pub struct UServices {}

#[tonic::async_trait]
impl UserService for UServices {
    async fn get_user(&self, req: Request<Id>) -> Result<Response<resp>, Status> {
        let id = req.into_inner().id;
        if let Some(user_to_be_displayed) =
            USER_DATA.write().unwrap().get_mut(&id.to_string()).cloned()
        {
            Ok(Response::new(resp {
                status: 1000,
                message: "Successfully got item".to_string(),
                data: serde_json::to_string_pretty(&user_to_be_displayed).unwrap(),
            }))
        } else {
            Ok(Response::new(resp {
                status: 4000,
                message: "Item not Found".to_string(),
                data: "".to_string(),
            }))
        }
    }
    async fn update_user(&self, req: Request<UserData>) -> Result<Response<resp>, Status> {
        let user_to_be_updated = req.into_inner();
        if let Some(item) = USER_DATA
            .write()
            .unwrap()
            .get_mut(&user_to_be_updated.id.to_string())
        {
            item.id = user_to_be_updated.id.clone();
            item.name = user_to_be_updated.name.clone();
            item.skills = user_to_be_updated.skills;
            item.status = string_to_status(user_to_be_updated.status.clone());
            item.language = string_to_language(user_to_be_updated.language.clone());

            // fs::write("./data/StudentData_Server.json", serde_json::to_string_pretty(&STUDENT_DATA.read().unwrap().clone()).unwrap());
            Ok(Response::new(resp {
                status: 4000,
                message: "Item updated".to_string(),
                data: serde_json::to_string_pretty(item).unwrap(),
            }))
        } else {
            Ok(Response::new(resp {
                status: 4000,
                message: "User not found".to_string(),
                data: "".to_string(),
            }))
        }
    }

    async fn delete_user(&self, req: Request<Id>) -> Result<Response<resp>, Status> {
        let id = req.into_inner().id;
        if let Some(employee_to_be_deleted) = USER_DATA.write().unwrap().remove(&id.to_string()) {
            Ok(Response::new(resp {
                status: 1000,
                message: "Item Deleted".to_string(),
                data: serde_json::to_string_pretty(&employee_to_be_deleted).unwrap(),
            }))
        } else {
            Ok(Response::new(resp {
                status: 4000,
                message: "Item not found".to_string(),
                data: "".to_string(),
            }))
        }
    }
    async fn create_user(&self, req: Request<UserData>) -> Result<Response<resp>, Status> {
        let user_to_be_created = req.into_inner();
        let user_to_be_added = create_user_struct(user_to_be_created);
        if !USER_DATA
            .read()
            .unwrap()
            .contains_key(&user_to_be_added.id.to_string())
        {
            USER_DATA.write().unwrap().insert(
                user_to_be_added.id.to_string().clone(),
                user_to_be_added.clone(),
            );
            Ok(Response::new(resp {
                status: 1000,
                message: "Item Created".to_string(),
                data: serde_json::to_string_pretty(&user_to_be_added).unwrap(),
            }))
        } else {
            Ok(Response::new(resp {
                status: 4000,
                message: "Item already exists".to_string(),
                data: "".to_string(),
            }))
        }
    }

    async fn get_all_user(&self, _req: Request<Empty>) -> Result<Response<resp>, Status> {
        let whole_data = USER_DATA.read().unwrap().clone();
        if !whole_data.is_empty() {
            Ok(Response::new(resp {
                status: 1000,
                message: "All items fetched".to_string(),
                data: serde_json::to_string_pretty(&whole_data).unwrap(),
            }))
        } else {
            Ok(Response::new(resp {
                status: 4000,
                message: "Please add data to the queue".to_string(),
                data: "".to_string(),
            }))
        }
    }
}

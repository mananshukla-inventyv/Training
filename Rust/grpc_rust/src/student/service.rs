use crate::helper_fun::{
    create_student_struct,
    ser::{
        student_service_server::StudentService, Empty, Id, Response as resp, StudentData as stdd,
    },
};

use tonic::{Request, Response, Result, Status};

use crate::data::STUDENT_DATA;

#[derive(Debug, Default)]
pub struct StdService {}

#[tonic::async_trait]
impl StudentService for StdService {
    async fn get_student(&self, req: Request<Id>) -> Result<Response<resp>, Status> {
        let id = req.into_inner().id;
        if let Some(student_to_be_displayed) = STUDENT_DATA
            .write()
            .unwrap()
            .get_mut(&id.to_string())
            .cloned()
        {
            Ok(Response::new(resp {
                status: 1000,
                message: "Successfully got item".to_string(),
                data: serde_json::to_string_pretty(&student_to_be_displayed).unwrap(),
            }))
        } else {
            Ok(Response::new(resp {
                status: 4000,
                message: "Item not Found".to_string(),
                data: "".to_string(),
            }))
        }
    }
    async fn update_student(&self, req: Request<stdd>) -> Result<Response<resp>, Status> {
        let student_to_be_updated = req.into_inner();
        if let Some(item) = STUDENT_DATA
            .write()
            .unwrap()
            .get_mut(&student_to_be_updated.id.to_string())
        {
            item.name = student_to_be_updated.name.clone();
            item.phone = student_to_be_updated.phone.clone();
            item.email = student_to_be_updated.email.clone();
            item.city = student_to_be_updated.city.clone();
            item.address = student_to_be_updated.address.clone();
            item.marks = student_to_be_updated.marks.clone();
            item.percent = student_to_be_updated.percent.clone();
            item.grade = student_to_be_updated.grade.clone();

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

    async fn delete_student(&self, req: Request<Id>) -> Result<Response<resp>, Status> {
        let id = req.into_inner().id;
        if let Some(student_to_be_deleted) = STUDENT_DATA.write().unwrap().remove(&id.to_string()) {
            Ok(Response::new(resp {
                status: 1000,
                message: "Item Deleted".to_string(),
                data: serde_json::to_string_pretty(&student_to_be_deleted).unwrap(),
            }))
        } else {
            Ok(Response::new(resp {
                status: 4000,
                message: "Item not found".to_string(),
                data: "".to_string(),
            }))
        }
    }
    async fn create_student(&self, req: Request<stdd>) -> Result<Response<resp>, Status> {
        let std_to_be_created = req.into_inner();
        let std_to_be_added = create_student_struct(std_to_be_created);
        if !STUDENT_DATA
            .read()
            .unwrap()
            .contains_key(&std_to_be_added.id.to_string())
        {
            STUDENT_DATA.write().unwrap().insert(
                std_to_be_added.id.to_string().clone(),
                std_to_be_added.clone(),
            );
            Ok(Response::new(resp {
                status: 1000,
                message: "Item Created".to_string(),
                data: serde_json::to_string_pretty(&std_to_be_added).unwrap(),
            }))
        } else {
            Ok(Response::new(resp {
                status: 4000,
                message: "Item already exists".to_string(),
                data: "".to_string(),
            }))
        }
    }

    async fn get_all_student(&self, _req: Request<Empty>) -> Result<Response<resp>, Status> {
        let whole_data = STUDENT_DATA.read().unwrap().clone();
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

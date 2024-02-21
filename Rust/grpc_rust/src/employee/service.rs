use crate::helper_fun::{create_employee_struct, ser::{employee_service_server::EmployeeService, EmployeeData as emp, Empty, Id, Response as resp}};

use tonic::{Request, Response, Result, Status};
use crate::data::EMPLOYEE_DATA;
use super::model::{ Position, Skills};


#[derive(Debug,Default)]
pub struct EmpService{}

#[tonic::async_trait]
impl EmployeeService for EmpService{
    async fn get_employee(&self,req:Request<Id>)->Result<Response<resp>,Status>{
        let id= req.into_inner().id;
        if let Some(employee_to_be_displayed)= EMPLOYEE_DATA.write().unwrap().get_mut(&id.to_string()).cloned(){
            Ok(Response::new(resp{
                status:1000,
                message:"Successfully got item".to_string(),
                data:serde_json::to_string_pretty(&employee_to_be_displayed).unwrap()
            }))
        }
        else {
            Ok(Response::new(resp{
                status:4000,
                message:"Item not Found".to_string(),
                data:"".to_string()
        }))
        }
    }
    async fn update_employee(&self,req:Request<emp>)->Result<Response<resp>,Status>{
        let employee_to_be_updated=req.into_inner();
        if let Some(item)=EMPLOYEE_DATA.write().unwrap().get_mut(&employee_to_be_updated.id.to_string()){

                item.id=employee_to_be_updated.id.clone();
                item.name=employee_to_be_updated.name.clone();
                item.age=employee_to_be_updated.age.clone();
                item.skills=vec![Skills::Python].clone();
                item.position=Some(Position::JuniorDeveloper).clone();
                item.experience=employee_to_be_updated.experience.clone();

                
                // fs::write("./data/StudentData_Server.json", serde_json::to_string_pretty(&STUDENT_DATA.read().unwrap().clone()).unwrap());
                Ok(Response::new(resp{
                    status:4000,
                    message:"Item updated".to_string(),
                    data:serde_json::to_string_pretty(item).unwrap()
                    
                }))
        }
        else {
            Ok(Response::new(resp{
                status:4000,
                message:"User not found".to_string(),
                data:"".to_string()
                
            }))
        }

    }

    async fn delete_employee(&self,req:Request<Id>)->Result<Response<resp>,Status>{
        let id= req.into_inner().id;
        if let Some(employee_to_be_deleted)= EMPLOYEE_DATA.write().unwrap().remove(&id.to_string()){
            Ok(Response::new(resp{
                status:1000,
                message:"Item Deleted".to_string(),
                data:serde_json::to_string_pretty(&employee_to_be_deleted).unwrap()  
            }))
        }
        else {
            Ok(Response::new(resp{
                status:4000,
                message:"Item not found".to_string(),
                data:"".to_string()
                
            }))
        }
    }
    async fn create_employee(&self,req:Request<emp>)->Result<Response<resp>,Status>{
        let emp_to_be_created=req.into_inner();
        let emp_to_be_added=create_employee_struct(emp_to_be_created);
        if !EMPLOYEE_DATA.read().unwrap().contains_key(&emp_to_be_added.id.to_string()){
            EMPLOYEE_DATA.write().unwrap().insert(emp_to_be_added.id.to_string().clone(), emp_to_be_added.clone());
            Ok(Response::new(resp{
                status:1000,
                message:"Item Created".to_string(),
                data:serde_json::to_string_pretty(&emp_to_be_added).unwrap() 
            }))
        }    
        else {
            Ok(Response::new(resp{
                status:4000,
                message:"Item already exists".to_string(),
                data:"".to_string()
            }))
        } 
    }

    async fn get_all_employee(&self,_req:Request<Empty>)->Result<Response<resp>,Status>{
        let whole_data=EMPLOYEE_DATA.read().unwrap().clone();
        if !whole_data.is_empty(){
            Ok(Response::new(resp{
                status:1000,
                message:"All items fetched".to_string(),
                data:serde_json::to_string_pretty(&whole_data).unwrap()
            }))
        }
        else {
            Ok(Response::new(resp{
                status:4000,
                message:"Please add data to the queue".to_string(),
                data:"".to_string()
                
            }))
        }
    }
    
}




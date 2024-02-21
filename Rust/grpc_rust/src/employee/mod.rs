use crate::helper_fun::ser::employee_service_server::EmployeeServiceServer;

use self::service::EmpService;

pub mod model;
pub mod service;

pub fn employee_services() -> EmployeeServiceServer<EmpService> {
    let my_server = EmpService::default();
    EmployeeServiceServer::new(my_server)
}

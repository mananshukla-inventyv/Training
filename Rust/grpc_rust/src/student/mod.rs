use crate::helper_fun::ser::student_service_server::StudentServiceServer;

use self::service::StdService;

pub mod model;
pub mod service;

pub fn student_services() -> StudentServiceServer<StdService> {
    let my_server = StdService::default();
    StudentServiceServer::new(my_server)
}

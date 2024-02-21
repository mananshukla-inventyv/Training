use crate::helper_fun::ser::user_service_server::UserServiceServer;

use self::service::UServices;


pub mod model;
pub mod service;


pub fn user_services()->UserServiceServer<UServices>{
    let my_server=UServices::default();
    UserServiceServer::new(my_server)
}
use grpc_server::{data::data_loader, employee::employee_services, student::student_services, user::user_services};
use tonic::transport::Server;



#[tokio::main]
async fn main()-> Result<(),Box<dyn std::error::Error>>{
    data_loader().await;
    let addr="127.0.0.1:50051".parse()?;
    
    Server::builder().add_service(student_services()).add_service(employee_services()).add_service(user_services()).serve(addr).await?;
    Ok(())
} 

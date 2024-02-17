use axum::extract::Path;

pub async fn get_item(Path(var):Path<(String,i32)>)->String{
    format!("got the {} with id: {}",var.0,var.1)
}
pub async fn create_item(Path(route_type):Path<String>)->String{
    format!("created the {}",route_type)
}
pub async fn delete_item(Path(var):Path<(String,i32)>)->String{
    format!("deleted the {} with id: {}",var.0,var.1)
}
pub async fn update_item(Path(var):Path<(String,i32)>)->String{
    format!("updated the {} with id: {}",var.0,var.1)
}
pub async fn get_all(Path(route_type):Path<String>)->String{
    format!("got all {}", route_type)
}
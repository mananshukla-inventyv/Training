use axum::{extract::Path, Json};

pub async fn path_variable(Path(var): Path<(i32, String)>) -> String {
    format!("{} and {}", var.0, var.1)
}

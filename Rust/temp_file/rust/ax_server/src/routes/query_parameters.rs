use std::collections::HashMap;

use axum::{extract::Query, Json};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct QueryParams {
    id: String,
    name: String,
}

pub async fn query_parameters(
    Query(query): Query<HashMap<String, String>>,
) -> Json<HashMap<String, String>> {
    Json(query)
}

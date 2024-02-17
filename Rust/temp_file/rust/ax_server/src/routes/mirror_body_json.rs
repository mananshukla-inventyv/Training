use axum::Json;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MirrorJson {
    str: String,
}

#[derive(serde::Serialize)]
pub struct MirrorJsonResp {
    str: String,
    msg_from_server: String,
}
pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResp> {
    Json(MirrorJsonResp {
        str: body.str,
        msg_from_server: "Hello from server".to_string(),
    })
}

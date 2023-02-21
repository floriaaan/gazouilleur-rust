use rocket::{
    http::Status,
    serde::json::{serde_json::json, Json, Value},
};

#[get("/")]
pub fn check() -> Result<Json<Value>, Status> {
    Ok(Json(json!({
        "status": "ok",
    })))
}

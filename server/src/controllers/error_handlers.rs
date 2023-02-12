use rocket::{
    serde::json::{serde_json::json, Json, Value},
    Request,
};

#[catch(404)]
pub fn not_found(req: &Request) -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Not Found",
        "message": "The requested resource could not be found but may be available again in the future.",
        "code": 404,

        "req": {
            "uri": req.uri().to_string(),
            "method": req.method().as_str(),
        }

    }))
}

#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Unprocessable Entity",
        "message": "The request was well-formed but was unable to be followed due to semantic errors.",
        "code": 422,

        "req": {
            "uri": req.uri().to_string(),
            "method": req.method().as_str(),
            
        }

    }))
}
#[catch(500)]
pub fn internal_error(req: &Request) -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": "Internal Server Error",
        "message": "The server encountered an unexpected condition that prevented it from fulfilling the request.",
        "code": 500,

        "req": {
            "uri": req.uri().to_string(),
            "method": req.method().as_str(),
        }
    }))
}

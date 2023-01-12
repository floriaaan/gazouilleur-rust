use crate::models::gazouilli::Gazouilli;
use rocket::{get, post, serde::json::Json};

#[get("/<id>")]
pub fn get(id: i32) -> Json<Gazouilli> {
    let gazouilli = Gazouilli {
        id,
        username: String::from("username"),
        content: String::from("content"),
        timestamp: chrono::Utc::now(),
    };
    return gazouilli.to_json();
}

#[get("/")]
pub fn get_all() -> Json<Vec<Gazouilli>> {
    return Json(vec![
        Gazouilli {
            id: 1,
            username: String::from("username"),
            content: String::from("content"),
            timestamp: chrono::Utc::now(),
        },
        Gazouilli {
            id: 2,
            username: String::from("username"),
            content: String::from("content"),
            timestamp: chrono::Utc::now(),
        },
    ]);
}

#[post("/", data = "<gazouilli>")]
pub fn create(gazouilli: String) {
    // code to save gazouilli to the database
}

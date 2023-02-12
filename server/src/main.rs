mod controllers;
mod libs;
mod models;
mod schema;
use controllers::{
    error_handlers::{internal_error, not_found, unprocessable_entity},
    gazouilli, user,
};

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json("{\"message\": \"Hello, world!\"}".to_string()))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount(
            "/gazouilli",
            routes![gazouilli::get, gazouilli::get_all, gazouilli::create],
        )
        .mount("/users", routes![user::get, user::create])
        .register(
            "/",
            catchers![not_found, internal_error, unprocessable_entity],
        )
}

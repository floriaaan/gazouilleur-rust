mod controllers;
mod libs;
mod models;
mod schema;
use controllers::{gazouilli, user};

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json, Request};

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
}

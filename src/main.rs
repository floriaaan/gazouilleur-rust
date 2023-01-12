mod controllers;
mod models;
use controllers::{gazouilli, user};

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from rust and mongoDB")))
}
// fn index() -> &'static str {
//     "Hello, world!"
// }

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

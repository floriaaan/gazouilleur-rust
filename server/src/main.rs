mod controllers;
mod libs;
mod models;
mod schema;
use std::error::Error;

use controllers::{
    error_handlers::{internal_error, not_found, unprocessable_entity},
    gazouilli, health, user,
};
use rocket::http::Method;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let allowed_origins = rocket_cors::AllowedOrigins::All;

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: rocket_cors::AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    let _ = rocket::build()
        .mount(
            "/gazouilli",
            routes![gazouilli::get, gazouilli::get_all, gazouilli::create],
        )
        .mount("/users", routes![user::get, user::create])
        .mount("/health", routes![health::check])
        .register(
            "/",
            catchers![not_found, internal_error, unprocessable_entity],
        )
        .attach(cors)
        .launch()
        .await?;

    Ok(())
}

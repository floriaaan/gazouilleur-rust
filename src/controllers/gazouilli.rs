use crate::libs::database::establish_connection;
use crate::models::gazouilli::{Gazouilli, NewGazouilli};
use rocket::{get, http::Status, post, serde::json::Json};

#[get("/<gazouilli_id>")]
pub fn get(gazouilli_id: i32) -> Result<Json<Gazouilli>, Status> {
    // todo: use singleton to get connection
    let connection = &mut establish_connection();
    let gazouilli = Gazouilli::get_by_id(connection, gazouilli_id);
    match gazouilli {
        Some(gazouilli) => Ok(Json(gazouilli)),
        None => Err(Status::NotFound),
    }
}

#[get("/")]
pub fn get_all() -> Result<Json<Vec<Gazouilli>>, Status> {
    // todo: use singleton to get connection
    let connection = &mut establish_connection();
    let gazouillis: Vec<Gazouilli> = Gazouilli::get_many(connection);
    return Ok(Json(gazouillis));
}

#[post("/", data = "<gazouilli>")]
pub fn create(gazouilli: Json<NewGazouilli>) -> Result<Json<Gazouilli>, Status> {
    // todo: use singleton to get connection
    let connection = &mut establish_connection();

    let gazouilli = Gazouilli::insert(connection, gazouilli.into_inner());

    match gazouilli {
        gazouilli => Ok(Json(gazouilli)),
    }
}

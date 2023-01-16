use crate::models::gazouilli::{Gazouilli, NewGazouilli};
use rocket::{get, post, serde::json::Json};
use crate::libs::database::establish_connection;

#[get("/<gazouilli_id>")]
pub fn get(gazouilli_id: i32) -> Json<Gazouilli> {
    // todo: use singleton to get connection
    let connection = &mut establish_connection();
    let gazouilli = Gazouilli::get_by_id(connection, gazouilli_id);
    return Json(gazouilli)
}

#[get("/")]
pub fn get_all() -> Json<Vec<Gazouilli>> {
    // todo: use singleton to get connection
    let connection = &mut establish_connection();
    let gazouillis: Vec<Gazouilli> = Gazouilli::get_many(connection);
    return Json(gazouillis);
}

#[post("/", data = "<gazouilli>")]
pub fn create(gazouilli: Json<NewGazouilli>) -> Json<Gazouilli> {
    // todo: use singleton to get connection
    let connection = &mut establish_connection();

    let gazouilli = Gazouilli::insert(connection, gazouilli.into_inner());
    return Json(gazouilli);
}

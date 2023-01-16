use rocket::{get, post, serde::json::Json};
use crate::models::user::User;

#[get("/<id>")]
pub fn get(id: i32) -> Json<User> {
    let user = User {
        id,
        name: String::from("username"),
    };
    return user.to_json();
}

#[post("/register", data = "<user_data>")]
pub fn create(user_data: String) {
    // code to register user
    println!("user_data: {}", user_data);
}

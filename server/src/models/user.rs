use rocket::serde::{json::Json, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

impl User {
    pub fn to_json(&self) -> Json<User> {
        Json(User {
            id: self.id,
            name: self.name.clone(),
        })
    }
}
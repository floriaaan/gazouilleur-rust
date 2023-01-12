use chrono::{DateTime, Utc};
use rocket::serde::{json::Json, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Gazouilli {
    pub id: i32,
    pub username: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

impl Gazouilli {
    pub fn to_json(&self) -> Json<Gazouilli> {
        Json(Gazouilli {
            id: self.id,
            username: self.username.clone(),
            content: self.content.clone(),
            timestamp: self.timestamp,
        })
    }
}
use crate::schema::gazouillis;
use chrono::NaiveDateTime;
use diesel::{
    prelude::{Identifiable, Queryable},
    query_dsl::methods::FilterDsl,
    ExpressionMethods, Insertable, PgConnection, RunQueryDsl,
};
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = gazouillis)]
pub struct Gazouilli {
    pub id: i32,
    pub username: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = gazouillis)]
pub struct NewGazouilli {
    pub username: String,
    pub content: String,
}

impl Gazouilli {
    pub fn to_json(&self) -> Json<Gazouilli> {
        Json(Gazouilli {
            id: self.id,
            username: self.username.clone(),
            content: self.content.clone(),
            created_at: self.created_at,
        })
    }

    pub fn get_by_id(
        connection: &mut PgConnection,
        gazouilli_id: i32,
    ) -> Option<Gazouilli> {
        use crate::schema::gazouillis::dsl::{gazouillis, id};

        return gazouillis
            .filter(id.eq(gazouilli_id))
            .first::<Gazouilli>(connection)
            .ok();
    }

    pub fn get_many(connection: &mut PgConnection) -> Vec<Gazouilli> {
        use crate::schema::gazouillis::dsl::gazouillis;

        return gazouillis
            .load::<Gazouilli>(connection)
            .expect("Error loading gazouillis");
    }

    pub fn insert(connection: &mut PgConnection, gazouilli: NewGazouilli) -> Gazouilli {
        use crate::schema::gazouillis::table as gazouillis_table;

        return diesel::insert_into(gazouillis_table)
            .values(gazouilli)
            .get_result(connection)
            .expect("Error saving new gazouilli");
    }
}

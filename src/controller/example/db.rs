use actix_web::{get, Responder, web::Data};
use serde::{Serialize, Deserialize};
use crate::AppState;
use crate::base::db::DbPool;
// use chrono::DateTime;
use sqlx::mysql::MySqlQueryAs;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Posts {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[get("/db/query")]
pub async fn query(state: Data<AppState>) -> impl Responder {
    // let rows: Vec<Posts> = vec![];

    let pool: &DbPool = &state.db;

    let result = sqlx::query_as::<_, Posts>("SELECT * from posts")
        .fetch_all(pool).await.unwrap();

    format!("DB Query {:?}", result)
}

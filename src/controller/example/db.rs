use actix_web::{get, post, Responder, web::{Data, Form}, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::AppState;
use crate::base::db::DbPool;
use crate::base::view::render;
// use chrono::DateTime;
use sqlx::mysql::MySqlQueryAs;
use tera::{Tera, Context};

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

#[get("/db/view/{id}")]
pub async fn view(req: HttpRequest, state: Data<AppState>) -> impl Responder {
    let id = req.match_info().get("id").unwrap_or("0");
    let id = id.parse::<u32>().unwrap();

    let row = get_post(id, &state.db).await;

    match row {
        Ok(v) => {
            format!("Get Row: {:?}", v)
        },
        Err(e) => {
            format!("Error: {}", e)
        }
    }
}

#[get("/db/add")]
pub async fn add(tmpl: Data<Tera>) -> impl Responder {
    let context = Context::new();
    render(&tmpl, "add.html", &context)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddPost {
    pub title: String,
    pub body: String
}

#[post("/db/add_post")]
pub async fn add_post(state: Data<AppState>, data: Form<AddPost>) -> impl Responder {
    let result = sqlx::query("INSERT INTO posts SET title=?,body=?,published=1").bind(&data.title).bind(&data.body).execute(&state.db).await;

    match result {
        Ok(affected) => {
            let id = get_last_id(&state.db).await;
            let row = get_post(id, &state.db).await.unwrap();
            format!("Insert {} rows: {:?}.", affected, row)
        },
        Err(e) => format!("Insert failed: {}", e)
    }
}

async fn get_post(id: u32, pool: &DbPool) -> Result<Posts, sqlx::Error> {
    let row = sqlx::query_as::<_, Posts>("SELECT * from posts WHERE id=?")
        .bind(id)
        .fetch_one(pool).await;

    match row {
        Ok(v) => Ok(v),
        Err(e) => Err(e)
    }
}

/// 获取最后插入的ID
async fn get_last_id(pool: &DbPool) -> u32 {
    sqlx::query_as::<_, (u32,)>("SELECT LAST_INSERT_ID() AS id").fetch_one(pool).await.unwrap().0
}
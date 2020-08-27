use actix_web::{get, HttpRequest, post, Responder, web::{Data, Form}};
use serde::{Deserialize, Serialize};
// use chrono::DateTime;
use sqlx::{mysql::MySqlQueryAs, FromRow};
use tera::{Context, Tera};

use crate::{
    AppState,
    base::{
        db::DbPool,
        rand::gen_rand,
        view::render
    }
};

#[derive(Serialize, Deserialize, Debug, FromRow)]
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

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Soul {
    pub id: u32,
    pub title: String,
    pub hits: u32
}

#[get("/db/soul")]
pub async fn show_soul(state: Data<AppState>, tmpl: Data<Tera>) -> impl Responder {
    let key = "soul_count";

    let count;

    if let Some(v) = state.cache.get::<u32>(key).await {
        count = v;
    } else {
        count = sqlx::query_as::<_, (i32,)>("SELECT COUNT(*) AS count FROM `soul`").fetch_one(&state.db).await.unwrap().0 as u32;
        state.cache.setex(key, &count, 600).await;
    }

    let pos = gen_rand(0, (count-1) as usize) as u32;

    let soul = sqlx::query_as::<_, Soul>("SELECT * FROM `soul` LIMIT 1 OFFSET ?").bind(pos).fetch_one(&state.db).await.unwrap();

    //更新命中次数
    sqlx::query("UPDATE `soul` SET `hits`=`hits`+1 WHERE `id`=?").bind(soul.id).execute(&state.db).await.unwrap();

    let mut context = Context::new();
    context.insert("soul", &soul);
    render(&tmpl, "soul.html", &context)
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
use actix_web::{get, Responder, web::Data};
use deadpool_redis::cmd;
use redis::RedisResult;
use crate::AppState;
use serde::{Serialize, Deserialize};

#[get("/redis/get")]
pub async fn getkey(state: Data<AppState>) -> impl Responder {
    let mut conn = state.redis.get().await.unwrap();
    let value: RedisResult<String> = cmd("GET")
        .arg(&["rusttest"])
        .query_async(&mut conn)
        .await;
    
   match value {
       Ok(v) => v,
       Err(e) => e.to_string()
   }
}

#[get("/redis/getcache")]
pub async fn getcache(state: Data<AppState>) -> impl Responder {
    let value = state.cache.get::<Storage>("rusttest").await;

    match value {
        Some(v) => format!("rusttest is: {:?}", v),
        None => "No Value".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Storage {
    key: u32,
    name: String,
    dist: bool
}

#[get("/redis/setcache")]
pub async fn setcache(state: Data<AppState>) -> impl Responder {
    let val = Storage {
        key: 123,
        name: String::from("Hello World"),
        dist: true
    };

    state.cache.setex("rusttest", &val, 10).await;


    let v2 = state.cache.get::<Storage>("rusttest").await;

    match v2 {
        Some(v) => format!("testset is: {:?}", v),
        None => "No Value".to_string()
    }
}


use actix_web::{get, Responder, web::Data};
use deadpool_redis::cmd;
use redis::RedisResult;
use crate::AppState;

#[get("/redis/get")]
pub async fn getkey(state: Data<AppState>) -> impl Responder {
    let mut conn = state.redis.get().await.unwrap();
    let value: RedisResult<String> = cmd("GET")
        .arg(&["hello_key"])
        .query_async(&mut conn)
        .await;
    
   match value {
       Ok(v) => v,
       Err(e) => e.to_string()
   }
}

#[get("/redis/getcache")]
pub async fn getcache(state: Data<AppState>) -> impl Responder {
    let value = state.cache.get_or_default::<String>("test", "Test World".to_string()).await;


    let v2 = state.cache.get("test2").await;

    let v2 = match v2 {
        Some(v) => v,
        None => "No Value".to_string()
    };

    format!("test is: {}, test2 is: {}", value, v2)
}

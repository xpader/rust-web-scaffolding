use sqlx::{Pool, MySqlPool, MySqlConnection};

pub type DbPool = Pool<MySqlConnection>;

pub async fn create_pool(uri: &String) -> DbPool {
    let pool = MySqlPool::builder()
        .max_size(5)
        .build(uri).await.unwrap();
    pool
}
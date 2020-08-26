use deadpool_redis::{cmd, Pool};
use redis::{FromRedisValue, RedisResult};
use serde::{de::DeserializeOwned, Deserialize};

pub struct Cache {
    pool: Pool
}

impl Cache {

    pub fn new(pool: Pool) -> Cache {
        Cache {
            pool
        }
    }

    pub async fn get_or_default<T>(&self, key: &str, default_value: T) -> T 
    where
        T: DeserializeOwned + std::marker::Send + FromRedisValue
    {
        let mut conn = self.pool.get().await.unwrap();
        let value: RedisResult<T> = cmd("GET")
            .arg(&[key])
            .query_async(&mut conn)
            .await;

        match value {
            Ok(v) => v,
            Err(_) => default_value
        }
    }

    pub async fn get<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned + std::marker::Send + FromRedisValue
    {
        let mut conn = self.pool.get().await.unwrap();
        let value: RedisResult<T> = cmd("GET")
            .arg(&[key])
            .query_async(&mut conn)
            .await;

        match value {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }

}

impl Clone for Cache {

    fn clone(&self) -> Cache {
        Cache {
            pool: self.pool.clone()
        }
    }

}

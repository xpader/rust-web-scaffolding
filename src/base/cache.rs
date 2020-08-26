use deadpool_redis::{cmd, Pool};
use redis::{FromRedisValue, RedisResult};
use serde::{de::DeserializeOwned, Serialize};

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
        let value: RedisResult<String> = cmd("GET")
            .arg(&[key])
            .query_async(&mut conn)
            .await;

        match value {
            Ok(v) => serde_json::from_str::<T>(v.as_str()).unwrap(),
            Err(_) => default_value
        }
    }

    pub async fn get<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned + std::marker::Send
    {
        let mut conn = self.pool.get().await.unwrap();
        let value: RedisResult<String> = cmd("GET")
            .arg(&[key])
            .query_async(&mut conn)
            .await;

        match value {
            Ok(v) => Some(serde_json::from_str::<T>(v.as_str()).unwrap()),
            Err(_) => None
        }
    }

    pub async fn set<T>(&self, key: &str, value: &T)
    where
        T: ?Sized + Serialize
    {
        let mut conn = self.pool.get().await.unwrap();

        let sval = serde_json::to_string(value).unwrap();

        cmd("SET").arg(&[key, &sval])
            .execute_async(&mut conn)
            .await.unwrap();
    }

    /// 设置有时效性的缓存
    ///
    /// 其中 `expires` 为有效时间秒数
    pub async fn setex<T>(&self, key: &str, value: &T, expires: u32)
    where
        T: ?Sized + Serialize
    {
        let mut conn = self.pool.get().await.unwrap();

        let sval = serde_json::to_string(value).unwrap();

        cmd("SETEX").arg(&[key, expires.to_string().as_str(), &sval])
            .execute_async(&mut conn)
            .await.unwrap();
    }

}

impl Clone for Cache {

    fn clone(&self) -> Cache {
        Cache {
            pool: self.pool.clone()
        }
    }

}

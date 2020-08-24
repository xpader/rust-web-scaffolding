use deadpool_redis::{Config, Pool};
use crate::base::app::Redis;
use std::env::set_var;

fn make_config(redis: &Redis) -> Config {
    set_var("REDIS_URL", &redis.url);

    if let Some(max_size) = redis.pool_max_size {
        set_var("REDIS_POOL.MAX_SIZE", max_size.to_string());
    }

    if let Some(timeout_secs) = redis.pool_timeout_secs {
        set_var("REDIS_POOL.TIMEOUTS.WAIT.SECS", timeout_secs.to_string());
    }
    
    if let Some(timeout_nanos) = redis.pool_timeout_nanos {
        set_var("REDIS_POOL.TIMEOUTS.WAIT.NANOS", timeout_nanos.to_string());
    }
    
    Config::from_env("REDIS").unwrap()
}

pub fn create_pool(redis: &Redis) -> Pool {
    let cfg = make_config(redis);
    let pool = cfg.create_pool().unwrap();
    pool
}

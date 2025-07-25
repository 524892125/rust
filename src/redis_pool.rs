use deadpool_redis::{Config, Pool, Runtime};

pub type RedisPool = Pool;

pub fn create_redis_pool() -> RedisPool {
    // let cfg = Config::from_url("redis://:your_password@192.168.9.128:6379/6");
    // let cfg = Config::from_url("redis://115.120.231.129/6");
    let cfg = Config::from_url("redis://:Eden@redis_1204@115.120.231.129:6379/99");
    cfg.create_pool(Some(Runtime::Tokio1)).unwrap()
}
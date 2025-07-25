use actix_web::{web, App, HttpServer};

mod modules;
mod redis_pool;
mod request;
mod logger;
mod response;
mod entity;

use crate::redis_pool::create_redis_pool;
use modules::version_service::get_value_from_redis;
use modules::version_cache::VersionCache;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe { std::env::set_var("RUST_LOG", "info"); }
    // 初始化 env_logger，读取环境变量设置日志等级
    logger::set_logger_format();

    let redis_pool = create_redis_pool(); // 先创建 RedisPool
    let version_cache = VersionCache::new();
    // 初始化缓存数据
    version_cache
        .load_from_redis(&redis_pool)
        .await
        .expect("Failed to load data from Redis");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))  // ✅ 注册共享数据
            .app_data(web::Data::new(version_cache.clone()))
            .service(get_value_from_redis)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
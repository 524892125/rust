use actix_web::{web, App, HttpServer};

mod modules;
mod redis_pool;

use crate::redis_pool::create_redis_pool;
use modules::version_service::get_value_from_redis;
use modules::version_cache::VersionCache;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8085");
    let redis_pool = create_redis_pool(); // 先创建 RedisPool
    let version_cache = VersionCache::new();
    // 初始化缓存数据
    version_cache
        .load_from_redis(&redis_pool)
        .await
        .expect("Failed to load data from Redis");

    // 打印缓存内容
    let cached_data = version_cache.get_channel_set().await;
    println!("Cached union_members:800100001 => {:?}", cached_data);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_pool.clone()))  // ✅ 注册共享数据
            .app_data(web::Data::new(version_cache.clone()))
            .service(get_value_from_redis)
    })
        .bind(("0.0.0.0", 8085))?
        .run()
        .await
}
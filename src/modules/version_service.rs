// src/modules/math.rs

use actix_web::{get, web, Responder, HttpResponse};
use crate::redis_pool::RedisPool;
use crate::modules::version_cache::VersionCache;

#[get("/get_value")]
pub async fn get_value_from_redis(
    cache: web::Data<VersionCache>,
) -> impl Responder {
    let values = cache.get_channel_set().await;
    if values.is_empty() {
        HttpResponse::NotFound().body("No cached values")
    } else {
        // 转换为 Vec 再 join
        let mut values: Vec<String> = values.into_iter().collect();
        values.sort(); // 如果你希望返回有序字符串（可选）
        HttpResponse::Ok().body(format!("Cached value: {}", values.join(",")))
    }
}

#[get("/refresh_cache")]
pub async fn refresh_cache(
    redis_pool: web::Data<RedisPool>,
    cache: web::Data<VersionCache>,
) -> impl Responder {
    match cache.refresh(&redis_pool).await {
        Ok(_) => HttpResponse::Ok().body("Cache refreshed"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Refresh failed: {}", e)),
    }
}


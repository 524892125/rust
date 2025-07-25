// src/modules/math.rs

use actix_web::{get, web, Responder, HttpResponse, post};
use crate::redis_pool::RedisPool;
use crate::modules::version_cache::VersionCache;
use crate::request::version_request::FormParams;
use crate::response::result::ApiResponse;
use crate::entity::version_info::VersionInfo;

#[post("/get_value")]
pub async fn get_value_from_redis(
    cache: web::Data<VersionCache>,
    form: web::Form<FormParams>
) -> impl Responder {
    println!("Received form: {:?}", form);  // ✅ 正确打印
    let channel_set = cache.get_channel_set().await;

    if !channel_set.contains(&form.channel) {
        return ApiResponse::<()>::fail(404, "Version not found");
    }

    let version_map = cache.get_version_map().await;

    let isWhite: bool = check_device_white(&form.deviceNo, cache).await;
    let key = format!("{}_{}_{}", form.channel, form.clientVersion, if isWhite {"pre"} else {"prod"});

    if let Some(version_str) = version_map.get(&key) {
        // 假设 value 是 JSON 字符串
        match serde_json::from_str::<VersionInfo>(version_str) {
            Ok(version_info) => {
                return ApiResponse::success(version_info);
            }
            Err(err) => {
                log::error!("Failed to parse version info JSON: {}", err);
                return ApiResponse::<()>::fail(201, "版本信息格式错误");
            }
        }
    } else {
        return ApiResponse::<()>::fail(201, "未找到对应版本信息");
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

pub async fn check_device_white (
    device_no: &str,
    cache: web::Data<VersionCache>,
) -> bool {
    let device_white_set = cache.get_device_white_set().await;
    device_white_set.contains(device_no)
}


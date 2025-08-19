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

    if !cache.has_channel(&form.channel) {
        return ApiResponse::<()>::fail(404, "Version not found");
    }

    let is_white = cache.is_device_white(&form.deviceNo);
    let key = format!("{}_{}_{}", form.channel, form.clientVersion, if is_white {"pre"} else {"prod"});

    if let Some(version_info) = cache.get_version_info(&key) {
        return ApiResponse::success(&*version_info);
    } else {
        return ApiResponse::<()>::fail(201, "未找到对应版本信息");
    }
}

// #[get("/refresh_cache")]
// pub async fn refresh_cache(
//     redis_pool: web::Data<RedisPool>,
//     cache: web::Data<VersionCache>,
// ) -> impl Responder {
//     match cache.refresh(&redis_pool).await {
//         Ok(_) => HttpResponse::Ok().body("Cache refreshed"),
//         Err(e) => HttpResponse::InternalServerError().body(format!("Refresh failed: {}", e)),
//     }
// }

// pub async fn check_device_white (
//     device_no: &str,
//     cache: web::Data<VersionCache>,
// ) -> bool {
//     let device_white_set = cache.device_white_set.await;
//     device_white_set.contains(device_no)
// }


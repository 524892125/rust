use std::sync::Arc;
use actix_web::{HttpMessage, HttpRequest};

pub fn get_request_id(req: &HttpRequest) -> Arc<String> {
    req
        .extensions()
        .get::<Arc<String>>() // 从 extensions 取
        .cloned()             // clone Arc（不会复制 String，只是增加引用计数）
        .unwrap_or_else(|| Arc::new("none".to_string()))
}
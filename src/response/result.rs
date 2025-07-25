use serde::Serialize;
use actix_web::{HttpResponse, http::StatusCode};

#[derive(Serialize)]
pub struct ApiResponse<T = ()> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> HttpResponse {
        HttpResponse::Ok().json(ApiResponse {
            code: 200,
            msg: "ok".to_string(),
            data: Some(data),
        })
    }

    pub fn success_no_msg(data: T) -> HttpResponse {
        HttpResponse::Ok().json(data)
    }

    pub fn fail(code: u16, msg: &str) -> HttpResponse {
        HttpResponse::Ok().json(ApiResponse::<String> {
            code,
            msg: msg.to_string(),
            data: Some(msg.to_string()),
        })
    }
}

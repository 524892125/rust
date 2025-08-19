use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::sync::Arc;
use std::task::{Context, Poll};
use uuid::Uuid;

/// RequestId 中间件结构体
pub struct RequestId;

impl RequestId {
    pub fn new() -> Self {
        RequestId
    }
}

// Transform trait: 用于包装 Service
impl<S, B> Transform<S, ServiceRequest> for RequestId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestIdMiddleware {
            service: std::sync::Arc::new(service),
        })
    }
}

pub struct RequestIdMiddleware<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        // 生成 RequestId，并用 Arc 包装
        let request_id = Arc::new(Uuid::new_v4().to_string());
        // 放入 request extensions
        req.extensions_mut().insert(request_id.clone());

        let srv = self.service.clone();
        Box::pin(async move {
            let mut res = srv.call(req).await?;
            // 添加到响应头
            res.headers_mut()
                .insert("X-Request-Id".parse().unwrap(), request_id.to_string().parse().unwrap());
            Ok(res)
        })
    }
}

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use uuid::Uuid;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::rc::Rc;

/// RequestId 中间件结构体
pub struct RequestId;

impl RequestId {
    pub fn new() -> Self {
        RequestId
    }
}

// 这里实现 Transform trait，用于包装 Service
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
            service: Rc::new(service),
        })
    }
}

pub struct RequestIdMiddleware<S> {
    service: Rc<S>,
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
        // 生成 UUID
        let request_id = Uuid::new_v4().to_string();
        // 插入 request extensions，供 handler 使用
        req.extensions_mut().insert(request_id.clone());

        // 也可以添加到响应头里
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut()
                .insert("X-Request-Id".parse().unwrap(), request_id.parse().unwrap());
            Ok(res)
        })
    }
}

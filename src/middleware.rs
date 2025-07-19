use axum::{
    extract::Request,
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;
use crate::CONFIG;

pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
) -> Result<Response<axum::body::Body>, impl IntoResponse> {
    // 获取客户端IP（暂时未使用，但保留用于未来扩展）
    let _client_ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .split(',')
        .next()
        .unwrap_or("unknown")
        .trim();

    // 创建限流器（从配置文件读取限制）
    let rate_limit = CONFIG.api_rate_limit.unwrap_or(100);
    let quota = Quota::per_minute(NonZeroU32::new(rate_limit as u32).unwrap());
    let rate_limiter = RateLimiter::direct(quota);

    // 检查是否超过限制
    if rate_limiter.check().is_ok() {
        Ok(next.run(req).await)
    } else {
        Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Rate limit exceeded. Please try again later.",
        ))
    }
} 
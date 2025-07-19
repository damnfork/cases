use axum::{
    extract::Request,
    http::{Response, StatusCode, HeaderMap},
    middleware::Next,
    response::IntoResponse,
};
use governor::{Quota, RateLimiter, state::{InMemoryState, NotKeyed}, clock::QuantaClock};
use std::{collections::HashMap, num::NonZeroU32, sync::Arc};
use crate::CONFIG;

// 定义具体的 RateLimiter 类型
type ConcreteRateLimiter = RateLimiter<NotKeyed, InMemoryState, QuantaClock>;

// 全局限流器缓存，按 token 存储不同的限流器
type TokenRateLimiters = Arc<std::sync::Mutex<HashMap<String, Arc<ConcreteRateLimiter>>>>;

static TOKEN_RATE_LIMITERS: std::sync::LazyLock<TokenRateLimiters> = 
    std::sync::LazyLock::new(|| Arc::new(std::sync::Mutex::new(HashMap::new())));

pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
) -> Result<Response<axum::body::Body>, impl IntoResponse> {
    let headers = req.headers();
    
    // 从请求头中获取 token
    let token = extract_token(headers);
    
    // 根据 token 获取限流配置
    let (rate_limit, token_key) = if let Some(token) = &token {
        if let Some(api_tokens) = &CONFIG.api_tokens {
            if let Some(token_config) = api_tokens.get(token) {
                (token_config.rate_limit, token.clone())
            } else {
                // token 无效
                return Err((
                    StatusCode::UNAUTHORIZED,
                    "Invalid API token",
                ));
            }
        } else {
            // 没有配置 token，使用默认限流
            (CONFIG.api_rate_limit.unwrap_or(100), "default".to_string())
        }
    } else {
        // 没有提供 token，使用默认限流
        (CONFIG.api_rate_limit.unwrap_or(100), "default".to_string())
    };

    // 获取或创建该 token 的限流器
    let rate_limiter = get_or_create_rate_limiter(&token_key, rate_limit);

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

/// 从请求头中提取 token
fn extract_token(headers: &HeaderMap) -> Option<String> {
    // 支持多种 token 传递方式
    
    // 1. Authorization: Bearer <token>
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }
    
    // 2. X-API-Token: <token>
    if let Some(token_header) = headers.get("x-api-token") {
        if let Ok(token_str) = token_header.to_str() {
            return Some(token_str.to_string());
        }
    }
    
    None
}

/// 获取或创建指定 token 的限流器
fn get_or_create_rate_limiter(token_key: &str, rate_limit: usize) -> Arc<ConcreteRateLimiter> {
    let mut limiters = TOKEN_RATE_LIMITERS.lock().unwrap();
    
    // 如果已存在该 token 的限流器，直接返回
    if let Some(limiter) = limiters.get(token_key) {
        return Arc::clone(limiter);
    }
    
    // 创建新的限流器
    let quota = Quota::per_minute(NonZeroU32::new(rate_limit as u32).unwrap());
    let rate_limiter = Arc::new(RateLimiter::direct(quota));
    
    // 缓存限流器
    limiters.insert(token_key.to_string(), Arc::clone(&rate_limiter));
    
    rate_limiter
} 
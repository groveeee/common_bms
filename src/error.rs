use axum::response::{IntoResponse, Response};
use serde::Serialize;
use std::fmt::{Display, Formatter};

/**
 * 错误枚举
 */
#[derive(Debug, Serialize)]
pub enum AppError {
    /// 未找到
    NotFound,
    /// 内部错误
    InnerError(&'static str),
    /// 数据库错误
    DatabaseError,
    /// 缓存错误
    CacheError,
    /// 校验错误
    ValidationError,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not Found"),
            AppError::InnerError(msg) => write!(f, "{}", msg),
            AppError::DatabaseError => write!(f, "Database Error"),
            AppError::CacheError => write!(f, "Cache Error"),
            AppError::ValidationError => write!(f, "Validation Error"),
        }
    }
}

/// 为AppError实现IntoResponse
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound => (axum::http::StatusCode::NOT_FOUND, "Not Found").into_response(),
            AppError::InnerError(msg) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            AppError::DatabaseError => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Database Error",
            )
                .into_response(),
            AppError::CacheError => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Cache Error").into_response()
            }
            AppError::ValidationError => {
                (axum::http::StatusCode::BAD_REQUEST, "Validation Error").into_response()
            }
        }
    }
}
// 实现其他错误类型到AppError的转换
impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::InnerError("IO Error")
    }
}
impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::DatabaseError
    }
}
impl From<std::num::ParseIntError> for AppError {
    fn from(value: std::num::ParseIntError) -> Self {
        AppError::ValidationError
    }
}
impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError::InnerError("Internal Server Error")
    }
}
impl From<redis::RedisError> for AppError {
    fn from(value: redis::RedisError) -> Self {
        AppError::CacheError
    }
}

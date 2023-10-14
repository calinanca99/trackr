pub use crate::routes::create_user::*;

use axum::{http::StatusCode, response::IntoResponse};

pub mod create_user;

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

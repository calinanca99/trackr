pub use crate::routes::add_link::*;
pub use crate::routes::create_user::*;
pub use crate::routes::login::*;

use axum::{http::StatusCode, response::IntoResponse};

pub mod add_link;
pub mod create_user;
pub mod login;

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health_check", get(health_check));

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

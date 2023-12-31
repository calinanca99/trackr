use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use trackr::routes::*;

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    let db = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("cannot connect to the DB");

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/api/v1/create_user", post(create_user))
        .route("/api/v1/login", post(login))
        .route("/api/v1/link", post(add_link))
        .with_state(db);

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

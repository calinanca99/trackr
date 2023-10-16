use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Days, Utc};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Login {
    username: String,
    password: String,
}

struct User {
    id: Uuid,
    password_hash: String,
}

pub async fn login(State(db): State<PgPool>, Json(login): Json<Login>) -> Response {
    let user = match get_user(&login.username, &db).await {
        Ok(u) => u,
        Err(sqlx::Error::RowNotFound) => {
            return (StatusCode::UNAUTHORIZED, "Credentials are incorrect").into_response()
        }
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let result = tokio::task::spawn_blocking(move || {
        let parsed_hash =
            PasswordHash::new(&user.password_hash).expect("cannot generate password hash");
        Argon2::default().verify_password(login.password.as_bytes(), &parsed_hash)
    })
    .await
    .expect("cannot spawn tokio task");

    if result.is_err() {
        return (StatusCode::UNAUTHORIZED, "Credentials are incorrect").into_response();
    }

    let session_id = Uuid::new_v4();
    let token = format!("trackr-{}", session_id);
    insert_session(session_id, user.id, token.as_str(), &db)
        .await
        .expect("cannot insert a session token");

    (StatusCode::OK, token).into_response()
}

async fn get_user(username: &str, db: &PgPool) -> Result<User, sqlx::Error> {
    let user = sqlx::query!(
        r#"
        select id, password_hash from users u
        where u.username = $1
    "#,
        username
    )
    .fetch_one(db)
    .await?;

    Ok(User {
        id: user.id,
        password_hash: user.password_hash,
    })
}

async fn insert_session(
    session_id: Uuid,
    user_id: Uuid,
    token: &str,
    db: &PgPool,
) -> Result<(), sqlx::Error> {
    let expires_at = Utc::now()
        .checked_add_days(Days::new(90))
        .expect("cannot compute expiring date for the token");

    sqlx::query!(
        r#"
        insert into user_sessions (id, user_id, token, expires_at) values ($1, $2, $3, $4)
    "#,
        session_id,
        user_id,
        token,
        expires_at
    )
    .execute(db)
    .await?;

    Ok(())
}

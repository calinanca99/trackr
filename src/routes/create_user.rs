use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{Account, Password, Username};

#[derive(Deserialize)]
pub struct CreateAccount {
    pub username: String,
    pub password: String,
}

pub async fn create_user(
    State(db): State<PgPool>,
    Json(create_account): Json<CreateAccount>,
) -> Response {
    match (
        Username::try_from(create_account.username),
        Password::try_from(create_account.password),
    ) {
        (Ok(username), Ok(password)) => {
            let user_id = Uuid::new_v4();
            match insert_user(user_id, username.to_string(), password.to_string(), &db).await {
                Ok(_) => {
                    let account = Account::new(username);
                    serde_json::to_string(&account).unwrap().into_response()
                }
                Err(e) => match e.as_database_error() {
                    Some(db_error) if db_error.is_unique_violation() => {
                        (StatusCode::CONFLICT, "Username is already taken").into_response()
                    }
                    _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                },
            }
        }
        _ => (
            StatusCode::BAD_REQUEST,
            "Password is not valid. A password must have at least 8 characters",
        )
            .into_response(),
    }
}

async fn insert_user(
    user_id: Uuid,
    username: String,
    password: String,
    db: &PgPool,
) -> Result<(), sqlx::Error> {
    let hash_task = tokio::task::spawn_blocking(move || {
        let argon2 = Argon2::default();

        let password_salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &password_salt)
            // TODO: Replace with `expect` or use `anyhow`
            .unwrap();

        (password_hash.to_string(), password_salt.to_string())
    })
    .await
    // TODO: Replace with `expect` or use `anyhow`
    .unwrap();

    sqlx::query!(
                    r#"
                insert into users (id, username, password_hash, password_salt) values ($1, $2, $3, $4)
                "#,
                    user_id,
                    username,
                    hash_task.0,
                    hash_task.1,
                )
                .execute(db)
                .await?;

    Ok(())
}

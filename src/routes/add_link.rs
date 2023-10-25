use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, TypedHeader,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::{DBResult, PgTransaction},
    domain::{Label, Name, URL},
};

#[derive(Deserialize, Serialize)]
pub struct NewLink {
    url: String,
    name: Option<String>,
    label: Option<String>,
}

struct User {
    id: Uuid,
}

pub async fn add_link(
    State(db): State<PgPool>,
    // TODO: Implement FromRequest(Parts) to extract the user id for endpoint
    // that require auth
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(new_link): Json<NewLink>,
) -> Response {
    let user_id = match get_user(token.token(), &db).await {
        Ok(user) => user.id,
        Err(sqlx::Error::RowNotFound) => {
            return (StatusCode::UNAUTHORIZED, "Token is invalid").into_response()
        }
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let link = URL::try_from(new_link.url.as_str());
    let name = new_link
        .name
        .clone()
        .map(|n| Name::try_from(n.as_str()))
        .transpose();
    let label = new_link
        .label
        .clone()
        .map(|l| Label::try_from(l.as_str()))
        .transpose();

    match (link, name, label) {
        (Ok(link), Ok(name), Ok(label)) => {
            let link_id = Uuid::new_v4();
            let mut tx = db.begin().await.expect("cannot start transaction");

            let res = match insert_link(link_id, user_id, link, name, &mut tx).await {
                Ok(_) => serde_json::to_string(&new_link).unwrap().into_response(),
                Err(e) => match e.as_database_error() {
                    Some(db_error) if db_error.is_unique_violation() => {
                        return (StatusCode::CONFLICT, "URL is saved already").into_response()
                    }
                    _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
                },
            };

            if let Some(link_label) = label {
                if insert_label(link_label, link_id, &mut tx).await.is_err() {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            }

            tx.commit().await.expect("cannot commit transaction");
            res
        }
        _ => (StatusCode::BAD_REQUEST, "Invalid URL").into_response(),
    }
}

async fn get_user(token: &str, db: &PgPool) -> DBResult<User> {
    let user = sqlx::query!(
        r#"
        select us.user_id from user_sessions us
        where us.token = $1 and us.expires_at > $2
        
    "#,
        token,
        Utc::now()
    )
    .fetch_one(db)
    .await?;

    Ok(User { id: user.user_id })
}

async fn insert_link(
    link_id: Uuid,
    user_id: Uuid,
    link: URL,
    name: Option<Name>,
    tx: &mut PgTransaction<'_>,
) -> DBResult<()> {
    sqlx::query!(
        r#"
        insert into links (id, user_id, link, link_name) values ($1, $2, $3, $4)
    "#,
        link_id,
        user_id,
        link.to_string(),
        name.map(|n| n.to_string())
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_label(
    link_label: Label,
    link_id: Uuid,
    tx: &mut PgTransaction<'_>,
) -> DBResult<()> {
    sqlx::query!(
        r#"
        insert into link_labels (link_label, link_id) values ($1, $2)
    "#,
        link_label.to_string(),
        link_id,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

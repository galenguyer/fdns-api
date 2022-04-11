use crate::db;
use crate::extractors;
use crate::routes::v1::requests;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use extractors::Json;
use serde_json::json;
use sqlx::{Error, Pool, Postgres};
use std::sync::Arc;

pub async fn create_user(
    Json(signup): Json<requests::Signup>,
    Extension(pool): Extension<Arc<Pool<Postgres>>>,
) -> impl IntoResponse {
    let user =
        db::users::create_user(&pool, &signup.email, &signup.password, &signup.display_name).await;
    match user {
        Ok(user) => (StatusCode::OK, Json(json!(user))),
        Err(err) => match err {
            Error::Database(e) if e.code().unwrap_or(std::borrow::Cow::Borrowed("")) == "23505" => {
                (StatusCode::BAD_REQUEST, Json(json!({"error": "A user with that email already exists"})))
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("{:?}", err) })),
            ),
        },
    }
}

pub async fn get_all_users(Extension(pool): Extension<Arc<Pool<Postgres>>>) -> impl IntoResponse {
    let users = db::users::get_all_users(&pool).await;
    match users {
        Ok(users) => return (StatusCode::OK, Json(json!(users))),
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": err.to_string()})),
            )
        }
    }
}

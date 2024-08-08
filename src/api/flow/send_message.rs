use crate::models::*;
use crate::{logged_user::LoggedUser, AppState};
use axum::routing::get;
use axum::Json;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Params {
    flow_id: Uuid
}

pub async fn create(
    LoggedUser(_user_id): LoggedUser,
    State(db): State<PgPool>,
    Json(params): Json<Params>
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    sqlx::query!(
        r#"insert into step (id, flow_id, t) 
        values ($1, $2, 'send_message') returning *"#,
        id , params.flow_id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let t = sqlx::query_as!(
        SendMessage,
        r#"insert into step_send_message (id, content) 
        values ($1, 'Hi, How can I help you?') returning *"#,
        id
    )
    .fetch_one(&db)
    .await;

    match t {
        Ok(t) => Json(json!(t)),
        Err(err) => {
            println!("{}", err);
            Json(json!({"error": err.to_string() }))
        }
    }
}

pub async fn get_one(
    LoggedUser(_user_id): LoggedUser,
    State(db): State<PgPool>,
    Path(id): Path<Uuid>
) -> Json<serde_json::Value> {
    let t = sqlx::query_as!(
        SendMessage,
        r#"select * from step_send_message t where id = $1 "#, id)
        .fetch_one(&db)
        .await
        .unwrap();

    Json(json!(t))
}

#[derive(Deserialize)]
pub struct UpdateParams {
    content: String,
}

pub async fn update(
    LoggedUser(_user_id): LoggedUser,
    State(db): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(params): Json<UpdateParams>,
) -> impl IntoResponse {
    sqlx::query!(
        "update step_send_message set content = $2 where id = $1",
        id,
        params.content
    )
    .execute(&db)
    .await
    .unwrap();

    Json(json!({}))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/step/send_message/:id", get(get_one).patch(update) )
        .route("/step/create_send_message", post(create))
}

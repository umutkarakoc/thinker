use crate::{logged_user::LoggedUser, AppState};
use crate::models::*;
use axum::routing::{post, patch};
use axum::{
    extract::{Path, State},
    response::IntoResponse, Json, routing::get, Router,
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
        values ($1, $2, 'wait_for_reply') returning *"#,
        id , params.flow_id
    )
    .fetch_one(&db)
    .await
    .unwrap();


    let t = sqlx::query_as!(
        WaitForReply,
        r#"insert into step_wait_for_reply (id) 
        values ($1) returning *"#,
        id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    sqlx::query!(
        "insert into step_wait_for_reply_branch 
            (parent_id, text) values 
            ($1, 'Answer 1'),
            ($1, 'Answer 2')",
        t.id
    )
        .execute(&db)
        .await
        .unwrap();


    Json(json!(t))
}

pub async fn get_one(
    LoggedUser(_user_id): LoggedUser,
    State(db): State<PgPool>,
    Path(id): Path<Uuid>
) -> Json<serde_json::Value> {
    let t = sqlx::query_as!(
        WaitForReply,
        "select * from step_wait_for_reply where id = $1 ",
        id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    Json(json!(t))
}

#[derive(Deserialize)]
pub struct BranchListParams {
    step_id: Uuid
}

pub async fn branch_list(
    LoggedUser(_user_id): LoggedUser,
    State(db): State<PgPool>,
    Json(params): Json<BranchListParams>
) -> Json<serde_json::Value> {

    let branches = sqlx::query_as!(WaitForReplyBrach,
        "select * from step_wait_for_reply_branch
        where parent_id = $1
        order by created_at, id desc",
        params.step_id)
        .fetch_all(&db)
        .await.unwrap();

    Json(json!(branches))
}

#[derive(Deserialize)]
pub struct AddBranchParams {
    ask_question_id: Uuid
}
pub async fn add_branch(
    LoggedUser(_user_id): LoggedUser,
    State(db): State<PgPool>,
    Json(params): Json<AddBranchParams>
) -> impl IntoResponse {
    let branch = sqlx::query_as!( WaitForReplyBrach,
        "insert into step_wait_for_reply_branch (parent_id, text) values ($1, 'Answer') returning *",
        params.ask_question_id,

    )
        .fetch_one(&db)
        .await
        .unwrap();

    Json(json!(branch))
}

#[derive(Deserialize)]
pub struct UpdateBranchParams {
    id: Uuid,
    text: String,
    contains: bool,
    fuzzy: bool
}
pub async fn update_branch(
    LoggedUser(_user_id): LoggedUser,
    State(db): State<PgPool>,
    Json(params): Json<UpdateBranchParams>
) -> impl IntoResponse {
    sqlx::query!(
        "update step_wait_for_reply_branch set text = $1, contains = $2, fuzzy = $3 where id = $4",
        params.text, params.contains, params.fuzzy, params.id
    ).execute(&db)
        .await
        .unwrap();

    Json(json!({}))
}


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/step/wait_for_reply_branch_list", post(branch_list))
        .route("/step/wait_for_reply/:id", get(get_one))
        .route("/step/create_wait_for_reply", post(create))
        .route("/step/add_wait_for_reply_branch", post(add_branch))
        .route("/step/update_wait_for_reply_branch", patch(update_branch))
}

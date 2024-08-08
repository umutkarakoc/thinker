use axum::{extract::State, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use crate::models::*;
use uuid::Uuid;

use crate::logged_user::LoggedUser;


#[derive(Deserialize)]
pub struct Params {
    flow_id: Uuid
}

pub async fn handler (
    LoggedUser(_user_id): LoggedUser,
    State(db): State<PgPool>,
    Json(params): Json<Params>
) -> impl IntoResponse {
    let steps = sqlx::query_as!(Step,
        r#"select * from step where flow_id = $1"#,
        params.flow_id
    ).fetch_all(&db)
        .await.unwrap();

    let connections = sqlx::query_as!(FlowConnection,
        r#"select c.* from flow_connection c
        join step s on s.id = c.to
        where s.flow_id = $1"#,
        params.flow_id
    ).fetch_all(&db)
        .await.unwrap();
    
    Json(json!({
        "steps": steps,
        "connections": connections
    }))
}
use crate::logged_user::LoggedUser;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Router, Json};
use sailfish::TemplateOnce;
use serde_json::json;
use sqlx::PgPool;

#[derive(TemplateOnce, Default)]
#[template(path = "flow/list.html")]
struct FlowListPage {
    _list: Vec<(String, String)> //id , name
}

async fn list(LoggedUser(user_id): LoggedUser, State(db): State<PgPool> ) -> impl IntoResponse {
    let list = sqlx::query!(r#"select * from flow where user_id = $1"#, user_id)
        .fetch_all(&db)
        .await.unwrap();

    let items : Vec<(String, String)> = list.iter()
        .map(|f| (f.id.to_string(), f.name.to_owned()))
        .collect();

    Json(json!({
        "items": items
    }))
}

pub fn router() -> Router<AppState> {
    Router::new().route("/flow", post(list))
}

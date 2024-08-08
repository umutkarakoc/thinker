use crate::{AppState, models::Flow};
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use crate::logged_user::LoggedUser;
use super::render_layout;

#[derive(TemplateOnce, Default)]
#[template(path = "automotion.html")]
pub struct PageTemplate {
    flow_list : Vec<Flow>
}

pub async fn page(
    LoggedUser(user_id):  LoggedUser, 
    State(db): State<PgPool>
) -> impl IntoResponse {
    let flow_list = sqlx::query_as!(Flow,
        "select c.*
        from flow c
        where user_id = $1
        order by c.name",
        user_id)
    .fetch_all(&db).await.unwrap();

    let page = PageTemplate {
        flow_list
    }.render_once().unwrap();

    render_layout(user_id, &db, "automotion", page).await.into_response()
}



pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(page))
}

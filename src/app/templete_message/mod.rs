use crate::{AppState};
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use crate::logged_user::LoggedUser;
use super::render_layout;

#[derive(TemplateOnce, Default)]
#[template(path = "template_message.html")]
pub struct PageTemplate {
    channel_list : Vec<(String, String, String, String, i64, i64)>
}


pub async fn page(
    LoggedUser(user_id):  LoggedUser, 
    State(db): State<PgPool>
) -> impl IntoResponse {


    let page = PageTemplate {
        channel_list: vec![]
    }.render_once().unwrap();

    render_layout(user_id, &db, "channel", page).await.into_response()
}


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(page))
}

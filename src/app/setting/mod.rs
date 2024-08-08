use crate::AppState;
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use crate::logged_user::LoggedUser;
use super::render_layout;

#[derive(TemplateOnce, Default)]
#[template(path = "setting.html")]
pub struct PageTemplate {
}


pub async fn page(
    LoggedUser(user_id):  LoggedUser, 
    State(db): State<PgPool>
) -> impl IntoResponse {
    let page = PageTemplate {}.render_once().unwrap();

    render_layout(user_id, &db, "setting", page).await.into_response()
}


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(page))
}

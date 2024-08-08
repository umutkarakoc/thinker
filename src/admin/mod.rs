mod login;
mod user;
use crate::{AppState};
use axum::{response::{IntoResponse, Html}, Router, routing::get};
use sailfish::TemplateOnce;
use crate::logged_user::LoggedAdmin;

#[derive(TemplateOnce)]
#[template(path = "admin/layout.html")]
pub struct Layout<Page: TemplateOnce> {
    page: Page
}


#[derive(TemplateOnce)]
#[template(path = "admin/dashboard.html")]
pub struct DashboadTemplate {
}

async fn home(LoggedAdmin(user_id): LoggedAdmin) -> impl IntoResponse {
    let dashboard = DashboadTemplate {};
    let layout = Layout { page: dashboard};

    Html(layout.render_once().unwrap())
}
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        .merge(user::router())
        .merge(login::router())
}

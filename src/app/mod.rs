mod automotion;
mod channel;
mod inbox;
mod flow;
mod setting;
mod social;
mod login;
mod templete_message;
use crate::{AppState, models::User};
use axum::{Router, response::{IntoResponse, Html, Redirect}, routing::get};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(TemplateOnce, Default)]
#[template(path = "layout.html")]
pub struct Layout {
    user: User,
    page: String,
    active_page: &'static str
}

pub async fn render_layout(
    user_id: Uuid,
    db: &PgPool,
    active_page: &'static str,
    page: String
) -> Html<String>{
    let user = sqlx::query_as!(User, r#"select * from "user" where id = $1 "#, user_id)
        .fetch_one(db).await.unwrap();

    Html (
        Layout {
            user,
            page,
            active_page
        }.render_once().unwrap()
    )
}

async fn home () -> impl IntoResponse {
    Redirect::to("/app/inbox").into_response()
}

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(flow::router())
        .merge(login::router())
        .route("/", get( home ) )
        .nest("/automotion", automotion::router())
        .nest("/broadcast", templete_message::router())
        .nest("/channel", channel::router())
        .nest("/inbox", inbox::router())
        .nest("/flow", flow::router())
        .nest("/setting", setting::router())
        .nest("/social", social::router())
}

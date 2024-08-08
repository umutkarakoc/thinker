mod chat;
mod contact_list;
use crate::AppState;
use axum::{Router, extract::{State, Path}, response::IntoResponse, routing::get};
use chrono::{DateTime, Utc};
use sailfish::TemplateOnce;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;
use crate::logged_user::LoggedUser;
use self::{contact_list::render_contact_list, chat::render_chat};
use super::render_layout;

#[derive(TemplateOnce, Default)]
#[template(path = "inbox/main.html")]
pub struct InboxTemplate {
    contact_list: String,
    chat: String
}

#[derive(Serialize)]
pub struct ContactItem {
    id: String,
    name: String,
    channel: String,
    channel_id: String,
    channel_type: String,
    text: Option<String>,
    message_type: String,
    last_message_at: DateTime<Utc>
}


pub async fn inbox(
    LoggedUser(user_id):  LoggedUser, 
    State(db): State<PgPool>
) -> impl IntoResponse {
    

    let page = InboxTemplate {
        contact_list: render_contact_list(&db, user_id, None).await,
        chat: r##"<div id="chat" class="p-4 is-flex is-justify-content-center is-align-items-center
        is-flex-direction-column m-0 has-background-info-light " style="flex: 3">
        <h3 class="is-size-4">Select a contact to chat</h3>
    </div>"##.to_string()
    }.render_once().unwrap();

    render_layout(user_id, &db, "inbox", page).await.into_response()
}

pub async fn inbox_selected(
    LoggedUser(user_id):  LoggedUser, 
    State(db): State<PgPool>,
    Path(selected): Path<Uuid>
) -> impl IntoResponse {
    

    let page = InboxTemplate {
        contact_list: render_contact_list(&db, user_id, Some(selected)).await,
        chat: render_chat(user_id, &db, selected).await
    }.render_once().unwrap();

    render_layout(user_id, &db, "inbox", page).await.into_response()
}


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(inbox))
        .route("/:contact", get(inbox_selected))
        .merge(contact_list::router())
        .merge(chat::router())
}

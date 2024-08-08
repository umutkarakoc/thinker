use crate::AppState;
use axum::{Router, extract::{State, Path}, response::{IntoResponse, Html}, routing::get};
use chrono::{DateTime, Utc};
use sailfish::TemplateOnce;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;
use crate::logged_user::LoggedUser;

#[derive(TemplateOnce, Default)]
#[template(path = "inbox/contact_list.html")]
pub struct InboxTemplate {
    contact_list: Vec<ContactItem>,
    selected: Option<Uuid>
}

#[derive(Serialize)]
pub struct ContactItem {
    id: Uuid,
    name: String,
    channel: String,
    ext_id: String,
    channel_id: String,
    channel_type: String,
    text: Option<String>,
    last_message_at: DateTime<Utc>
}

pub async fn render_contact_list (db: &PgPool, user_id: Uuid, selected: Option<Uuid>) -> String {
    let contact_list = sqlx::query_as!(ContactItem,
    r#"select co.id, co.name, co.ext_id,
            ch.name as channel, ch.id as channel_id , ch.t as channel_type,
            txt.text as "text?", 
            last_message.created_at as last_message_at
        from contact co
        join channel ch on ch.id = co.channel_id
        join (select DISTINCT ON (contact_id) message.*
            from message
            order by contact_id, created_at desc
          ) as last_message 
            on co.id = last_message.contact_id
        left join message_text as txt on txt.id = last_message.id
        where ch.owner_id = $1
        order by last_message.created_at desc "#,
        user_id)
        .fetch_all(db)
        .await.unwrap();

    println!("count: {}", contact_list.len());

    InboxTemplate {
        contact_list,
        selected
    }.render_once().unwrap()
}

pub async fn contact_list(
    LoggedUser(user_id):  LoggedUser, 
    State(db): State<PgPool>,
    Path(selected): Path<Uuid>
) -> impl IntoResponse {
    Html(
        render_contact_list(&db, user_id, Some(selected) ).await
    ).into_response()
}


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/contact_list/:contact", get(contact_list))
}

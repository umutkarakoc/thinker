use crate::logged_user::LoggedUser;
use crate::models::MessageType;
use crate::service;
use crate::{
    models::{Channel, Contact, Flow, Message, MessageMedia, MessageText},
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use hyper::StatusCode;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::{postgres::PgRow, FromRow, PgPool};
use uuid::Uuid;

#[derive(TemplateOnce)]
#[template(path = "inbox/message_list.html")]
pub struct MessageListTemplate {
    message_list: Vec<Message>,
}

#[derive(TemplateOnce, Deserialize, Serialize)]
#[template(path = "inbox/chat.html")]
pub struct ChatBoxTemplate {
    message_list_render: String,
    contact_id: String,
    ext_id: String,
    name: String,
    flow: Option<(Uuid, String)>,
    flows: Vec<Flow>,
}

pub async fn render_chat(user_id: Uuid, db: &PgPool, contact_id: Uuid) -> String {
    let contact = sqlx::query!(
        r#"select contact.id, contact.name, contact.channel_id, contact.ext_id,
            flow.name as "flow_name?", flow.id as "flow_id?"
            from contact
            left join (select DISTINCT ON (contact_id) contact_step.*
                    from contact_step
                    order by contact_id, created_at desc
                  ) as cs
                    on contact.id = cs.contact_id
            left join step on step.id = cs.step_id
            left join flow on flow.id = step.flow_id
        where contact.id = $1"#,
        contact_id
    )
    .fetch_one(db)
    .await
    .unwrap();

    let message_list = sqlx::query(
        r#"select m.*, txt.text as "text", 
            media.url as "media_url", media.media_type as "media_type" , media.text as "media_text"
        from message m
        left join message_text txt on txt.id = m.id
        left join message_media media on media.id = m.id
        where contact_id = $1 
        order by created_at desc
        limit 50"#,
    )
    .bind(contact_id)
    .map(|row| Message::from_row(&row).unwrap())
    .fetch_all(db)
    .await
    .unwrap();

    let flows = sqlx::query_as!(Flow, "select * from flow where user_id = $1", user_id)
        .fetch_all(db)
        .await
        .unwrap();

    let message_list_render = MessageListTemplate { message_list }.render_once().unwrap();

    let flow = match (contact.flow_id, contact.flow_name) {
        (Some(id), Some(name)) => Some((id, name)),
        _ => None,
    };

    ChatBoxTemplate {
        flows,
        message_list_render,
        contact_id: contact.id.to_string(),
        ext_id: contact.ext_id,
        name: contact.name,
        flow,
    }
    .render_once()
    .unwrap()
}

pub async fn render_messages(db: &PgPool, contact_id: Uuid) -> String {

    let message_list = sqlx::query!(
        r#"select m.*, txt.text as "text?", 
            media.url as "media_url?", media.media_type as "media_type?" , media.text as "media_text?"
        from message m
        left join message_text txt on txt.id = m.id
        left join message_media media on media.id = m.id
        where contact_id = $1 
        order by created_at desc
        limit 50"#,
        contact_id
    )
    .fetch_all(db)
    .await
    .unwrap()
    .iter().to_owned()
    .map(|row| Message {
        id: row.id,
        status: row.status.clone(),
        created_at: row.created_at.clone(),
        created_by: row.created_by.clone(),
        reply_for: row.reply_for.clone(),
        contact_id: row.contact_id,
        mid: row.mid.clone(),
        t: match row.t.as_str() {
            "text" => MessageType::Text(row.text.clone().unwrap()),
            "media" => MessageType::Media(MessageMedia {
                url: row.media_url.clone().unwrap(),
                media_type: row.media_type.clone().unwrap(),
                text: row.media_text.clone()
            }),
            _ => MessageType::Text("".to_string()),
        },
    }).collect();


    MessageListTemplate { message_list }.render_once().unwrap()
}

#[derive(Deserialize)]
pub struct ChatQuery {
    _after: Option<String>,
}

pub async fn chat(
    LoggedUser(user_id): LoggedUser,
    State(db): State<PgPool>,
    Path(selected): Path<Uuid>,
    Query(ChatQuery { _after }): Query<ChatQuery>,
) -> impl IntoResponse {
    Html(render_chat(user_id, &db, selected).await).into_response()
}

#[derive(Deserialize)]
pub struct TextMessage {
    pub text: String,
}

pub async fn text(
    LoggedUser(user_id): LoggedUser,
    State(db): State<PgPool>,
    Path(contact_id): Path<Uuid>,
    Form(params): Form<TextMessage>,
) -> impl IntoResponse {
    let channel = sqlx::query_as!(
        Channel,
        "select c.* from channel c
        join contact co on co.channel_id = c.id
        where co.id = $1 and c.owner_id = $2",
        contact_id,
        user_id
    )
    .fetch_one(&db)
    .await;

    match channel {
        Ok(channel) => {
            let contact =
                sqlx::query_as!(Contact, "select * from contact where id = $1", contact_id)
                    .fetch_one(&db)
                    .await
                    .unwrap();

            let _msg = service::channel::send_text(contact_id, user_id, params.text, &db);

            Html(render_chat(user_id, &db, contact_id).await).into_response()
        }
        Err(_) => StatusCode::FORBIDDEN.into_response(),
    }
}

#[derive(Deserialize)]
pub struct AssignFlowParams {
    flow_id: String,
}
pub async fn assign_flow(
    LoggedUser(user_id): LoggedUser,
    State(db): State<PgPool>,
    Path(contact_id): Path<Uuid>,
    Form(params): Form<AssignFlowParams>,
) -> impl IntoResponse {
    if params.flow_id.is_empty() {
        sqlx::query!(
            r#"insert into "contact_step"
            (step_id, contact_id)
             values (null, $1)  "#,
            contact_id
        )
        .execute(&db)
        .await
        .unwrap();
    } else {
        let flow_id = Uuid::parse_str(&params.flow_id).unwrap();
        let con = sqlx::query!(
            r#"select c.to from flow_connection c
            join flow f on f.id = c.id
            where f.id = $1 and f.user_id = $2"#,
            flow_id,
            user_id
        )
        .fetch_one(&db)
        .await
        .unwrap();

        sqlx::query!(
            r#"insert into "contact_step"
            (step_id, contact_id)
             values ($1, $2)  "#,
            con.to,
            contact_id
        )
        .execute(&db)
        .await
        .unwrap();
    }

    (StatusCode::OK).into_response()
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/chat/:contact/message_list", get(chat))
        .route("/chat/:contact/text", post(text))
        .route("/chat/:contact/flow", post(assign_flow))
}

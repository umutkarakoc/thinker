extern crate hyper;
extern crate hyper_native_tls;

use axum::{
    response::IntoResponse,
    Json,
};

use axum::extract::{Path, State};
use hyper::StatusCode;
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use crate::REQWEST;

pub async fn save_message(db: &PgPool, channel_id: &str, msg: &Value) -> Option<bool>{
    let t = msg.get("type")?.as_str()?;
    let mid = msg.get("id")?.as_str()?;
    let contact_ext_id = msg.get("from")?.as_str()?;
    let reply_for = msg.get("context").map_or(None, |context| context.get("id").map_or(None, |id| id.as_str()) );
    let id = Uuid::new_v4();

    sqlx::query!("insert into message 
        (id, mid, contact_id, reply_for, status, created_by) 
        values ($1, $2, (select id from contact where ext_id = $3 and channel_id = $4 ) , $5, 'received' , 'contact')
        on conflict do nothing",
        id, mid, contact_ext_id, channel_id, reply_for).execute(db).await;

    match t {
        "text" => {
            let text = msg.get("text")?.get("body")?.as_str()?;

            sqlx::query!("insert into message_text
                (id, text) values ($1, $2)
                on conflict do nothing",
                id, text)
            .execute(db).await.unwrap();
        },
        "image" | "video" | "document" | "voice" | "audio" => {
            let media = msg.get(t)?.as_object()?;
            let media_id = media.get("id")?.as_str()?;
            let text = media.get("caption").map_or(None, |cap| cap.as_str());

            let url = format!("/storage/{}/{}", channel_id, media_id);

            sqlx::query!("insert into message_text
                (id, text) values ($1, $2)
                on conflict do nothing",
                id, text)
            .execute(db).await.unwrap();

            sqlx::query!("insert into message_media
                (id, url, media_type) values ($1, $2, $3)
                on conflict do nothing",
                id, url, t)
            .execute(db).await.unwrap();
        },
        _ => {
            return None;
        }
    }

    return Some(true);
}

pub async fn save_customer(db: &PgPool, channel_id: &str, c: &Value) -> Option<bool> {
    let ext_id =  c.get("wa_id")?.as_str()?;
    let name =  c.get("profile")?.as_object()?.get("name")?.as_str()?;

    sqlx::query!("insert into contact 
        (ext_id, channel_id, name) 
        values ($1, $2, $3)
        on conflict do nothing ",
        ext_id,  channel_id, name)
        .execute(db).await.unwrap();

    Some(true)
}

pub async fn save_status(db: &PgPool, status: &Value) -> Option<bool> {
    let id =  status.get("id")?.as_str()?;
    let status =  status.get("status")?.as_str()?;

    sqlx::query!("update message set status = $2 where mid = $1", id, status)
        .execute(db).await.unwrap();

    Some(true)
}

pub async fn whatsapp(
    State(db): State<PgPool>,
    Path(channel_id): Path<String>, 
    Json(params): Json<Value>) -> impl IntoResponse {

    let uri = format!("http://127.0.0.1:3000/api/webhook/whatsapp/{}", channel_id); //sonitel

    let _res = REQWEST
        .post(uri)
        .header("content-type", "application/json")
        .body(params.to_string())
        .send()
        .await;
    
    let contacts = params.get("contacts");
    match contacts {
        Some(contacts) => { 
            let contacts = contacts.as_array().unwrap();
            for c in contacts {
                save_customer(&db, &channel_id, c)
                    .await;
            }
        },
        None => {}
    }

    let messages = params.get("messages");
    match messages {
        Some(messages) => {
            let messages = messages.as_array().unwrap();
            for msg in messages {
                save_message(&db, &channel_id, msg)
                    .await;
            }
        },
        None => {}
    }

    let statuses = params.get("statuses");
    match statuses {
        Some(statuses) => { 
            let statuses = statuses.as_array().unwrap();
            for status in statuses {
                save_status(&db, status)
                    .await;
            }
        },
        None => {}
    }


    StatusCode::OK.into_response()
}

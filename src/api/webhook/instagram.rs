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

use crate::models::Contact;

pub async fn save_customer(db: &PgPool, channel_id: &str, c: &Value) -> Option<bool> {
    let ext_id =  c.get("wa_id")?.as_str()?;
    let name =  c.get("profile")?.as_object()?.get("name")?.as_str()?;

    Some(true)
}


pub async fn save_message(db: &PgPool,  msg: &Value) -> Option<bool>{
    let ext_id = msg.get("sender")?.get("id")?.as_str()?;
    let channel_id = msg.get("recipient")?.get("id")?.as_str()?;
    let mid = msg.get("message")?.get("mid")?.as_str()?;
    let text = msg.get("message")?.get("text");
    let id = Uuid::new_v4();
    let t = if text.is_some() { "text" } else { "media" };

    sqlx::query!("insert into contact 
        (ext_id, channel_id, name) 
        values ($1, $2, $3)
        on conflict do nothing ",
        ext_id,  channel_id, "")
        .execute(db).await;

    sqlx::query!("insert into message 
        (id, mid, t, contact_id, status, created_by) 
        values ($1, $2, $5, (select id from contact where ext_id = $3 and channel_id = $4 ) , 'received' , 'contact' )
        on conflict do nothing",
        id, mid, ext_id, channel_id, t).execute(db).await.unwrap();

    match text {
        Some(text) => {
            let text = text.as_str();

            sqlx::query!("insert into message_text
                (id, text) values ($1, $2)
                on conflict do nothing",
                id, text)
            .execute(db).await.unwrap();
        },
        _ => {
            let media = msg.get("message")?.get("attachments")?.as_array()?.get(0)?;
            let t = media.get("type")?.as_str()?;
            let url = media.get("payload")?.get("url")?.as_str()?;

            sqlx::query!("insert into message_media
                (id, url, media_type) values ($1, $2, $3)
                on conflict do nothing",
                id, url, t)
            .execute(db).await.unwrap();
        }
    }

    return Some(true);
}

pub async fn save_status(db: &PgPool, status: &Value) -> Option<bool> {
    let id =  status.get("id")?.as_str()?;
    let status =  status.get("status")?.as_str()?;

    sqlx::query!("update message set status = $2 where mid = $1",id, status)
        .execute(db).await.unwrap();

    Some(true)
}

pub async fn instagram(db: &PgPool, params: &Value) {
    let entry = params.get("entry").map(|entry| entry.as_array());

    if let Some(Some(entry)) = entry  {
        for e in entry.iter() {
             let messaging = e.get("messaging").map(|m| m.as_array());
             if let Some(Some(messaging)) = messaging {
                for msg in messaging { 
                    save_message(db, msg).await;
                }
             }
        }
    }
}

use std::ops::Index;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;
use crate::REQWEST;

pub async fn send_text (
	contact_id: Uuid,
	created_by: Uuid, 
	text: String,
	db: &PgPool) -> Option<Uuid> {
	//todo: other channel rype


	let contact = sqlx::query!("select * from contact where id = $1", contact_id)
		.fetch_one(db).await.unwrap();

	let wa_channel = sqlx::query!("select * from channel_whatsapp where id = $1", contact.channel_id)
		.fetch_one(db).await.unwrap();

    let body = json!({
	    "to": contact.ext_id,
	    "recipient_type": "individual",
	    "type": "text",
	    "text": {
	        "body": text
	    }
	});
	let result = REQWEST.post(format!("{}/v1/messages", wa_channel.host))
	    .header("Authorization", format!("Bearer {}", wa_channel.token))
	    .json(&body)
	    .send()
	    .await.unwrap();

	let result = result.json::<Value>().await.unwrap();

	let mid = result.get("messages").unwrap()
	    .as_array().unwrap()
	    .index(0).get("id")
	    .unwrap().as_str().unwrap().to_string();

	let id = Uuid::new_v4();

	sqlx::query!("insert into message 
        (id, mid, contact_id, status, created_by) 
        values ($1, $2, $3, 'sent' , $4)
        on conflict do nothing",
        id, mid, contact.id, created_by.to_string()).execute(db).await;

    sqlx::query!("insert into message_text
                (id, text) values ($1, $2)
                on conflict do nothing",
                id, text)
            .execute(db).await.unwrap();

	Some(id)
}
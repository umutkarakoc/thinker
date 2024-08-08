use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, types::Json, postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
}

#[derive(Deserialize, Clone, FromRow, Debug)]
pub struct Admin {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub deleted: bool,
}

#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct Channel {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub owner_id: Uuid,
    pub name: String,
    pub t: String
}

#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct ChannelWhatsapp {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub host: String,
    pub name: String,
    pub token: String,
    pub token_expire_at: DateTime<Utc>,
    pub password: String,
    pub state: String,
    pub cert: String
}

#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct ChannelInstagram {
    pub id: String,
    pub token: String,
    pub cert: String
}

#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct ChannelFacebook {
    pub id: String,
    pub token: String,
    pub cert: String
}


#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct  WebHook {
    pub id: i64,
    pub t: String,
    pub data: Json<Value>,
    pub created_at: DateTime<Utc>
}

pub struct Message {
    pub id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub reply_for: Option<String>,
    pub contact_id: Uuid,
    pub mid: String,
    pub t: MessageType
}

pub enum MessageType {
    Text(String),
    Media(MessageMedia)
}
#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct MessageText {
    pub id: Uuid,
    pub text: String,
}

#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct MessageMedia {
    pub url: String,
    pub media_type: String,
    pub text: Option<String>,
}


impl FromRow<'_, PgRow> for Message {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let t : &str = row.try_get("t")?;

        let t = match t {
            "text" => MessageType::Text( row.try_get("text")? ),
            "media" => MessageType::Media( MessageMedia { 
                url: row.try_get("media_url")?, 
                media_type: row.try_get("media_type")? , 
                text: row.try_get("media_text")?
            } ),
            _ => MessageType::Text( "".to_string() )
        };

        Ok(Self {
           id: row.try_get("id")?,
           status: row.try_get("status")?,
           created_at: row.try_get("created_at")?,
           created_by: row.try_get("created_by")?,
           reply_for: row.try_get("reply_for")?,
           contact_id: row.try_get("contact_id")?,
           mid: row.try_get("mid")?,
           t
        })
    }
}

#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct Contact {
    pub id: Uuid,
    pub ext_id: String,
    pub channel_id: String,
    pub name: String,
    pub created_at: DateTime<Utc>
}

#[derive(Deserialize, Clone, FromRow, Debug, Default)]
pub struct ContactStep {
    pub id: Uuid,
    pub step_id: Uuid,
    pub contact_id: Uuid,
    pub log: Option<String>,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub next_id: Option<Uuid>,
    pub meta: Option<Json<Value>>
}


#[derive(Serialize, Deserialize)]
pub struct Flow {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub user_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct FlowConnection {
    pub id: Uuid,
    pub to: Uuid
}


#[derive(Serialize, Deserialize)]
pub struct Step {
    pub id: Uuid,
    pub t: String,
    pub created_at: DateTime<Utc>,
    pub flow_id: Uuid
}


#[derive(Serialize, Deserialize)]
pub struct SendMessage {
    pub id: Uuid,
    pub content: String
}

#[derive(Serialize, Deserialize)]
pub struct SendMediaMessage {
    pub id: Uuid,
    pub url: String,
    pub header: Option<String>,
    pub content: Option<String>,
    pub footer: Option<String>
}

// #[derive(Serialize, Deserialize)]
// pub struct AskQuestion {
//     pub id: Uuid,
//     pub content: String,
//     pub other_next_id: Option<Uuid>,
//     pub created_at: DateTime<Utc>,
//     pub flow_id: Uuid
// }

// #[derive(Serialize, Deserialize)]
// pub struct AskQuestionReply {
//     pub id: Uuid,
//     pub parent_id: Uuid,
//     pub text: String,
//     pub contains: bool,
//     pub fuzzy: bool,
//     pub next_id: Option<Uuid>,
// }

#[derive(Serialize, Deserialize)]
pub struct WaitForReply {
    pub id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct WaitForReplyBrach {
    pub id: Uuid,
    pub parent_id: Uuid,
    pub text: String,
    pub contains: bool,
    pub smart: bool,
    pub fuzzy: bool,
    pub created_at: DateTime<Utc>
}

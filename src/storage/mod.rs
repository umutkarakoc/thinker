use crate::{AppState, models::{User, Channel}, REQWEST, logged_user::LoggedUser};
use axum::{Router, response::{IntoResponse}, routing::get, extract::{Path, State}};
use sailfish::TemplateOnce;
use sqlx::PgPool;



#[derive(TemplateOnce, Default)]
#[template(path = "layout.html")]
pub struct Layout {
    user: User,
    page: String,
    active_page: &'static str
}

pub async fn get_file(
    LoggedUser(user_id): LoggedUser,
    State(db): State<PgPool>,
    Path((channel, media)): Path<(String, String)>
) -> impl IntoResponse {
    //todo check channel owner too
    let channel = sqlx::query!("select * from channel_whatsapp where id = $1",channel)
        .fetch_one(&db).await.unwrap();

    let result = REQWEST.get(format!("{}/v1/media/{}", channel.host, media))
        .header("Authorization", format!("Bearer {}", channel.token))
        .send()
        .await.unwrap();

    result.bytes().await.unwrap()
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:channel/:media", get( get_file ) )
}

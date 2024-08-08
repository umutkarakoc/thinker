use crate::{AppState};
use axum::{Router, extract::State, response::IntoResponse, routing::get};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use crate::logged_user::LoggedUser;
use super::render_layout;

#[derive(TemplateOnce, Default)]
#[template(path = "channel.html")]
pub struct PageTemplate {
    channel_list : Vec<(String, String, String, String, i64, i64)>
}


pub async fn page(
    LoggedUser(user_id):  LoggedUser, 
    State(db): State<PgPool>
) -> impl IntoResponse {
    let channel_list = sqlx::query!(
        "select c.*, 
            (select count(m.id) 
                from message m 
                join contact co on co.channel_id = c.id
                where co.channel_id = c.id 
            ) as message_count , 

            (select count(co.id) from contact co where co.channel_id = c.id ) as contact_count
        from channel c
        where c.owner_id = $1
        group by c.id
        order by c.created_at desc",
        user_id)
    .fetch_all(&db).await.unwrap()
    .iter().map(|c| {
        (
            c.id.to_string(),
            c.name.to_string(),
            c.t.to_string(),
            c.created_at.format("%H:%M %d/%m/%Y").to_string(),
            c.contact_count.unwrap_or(0),
            c.message_count.unwrap_or(0),
        )
    }).collect();

    let page = PageTemplate {
        channel_list
    }.render_once().unwrap();

    render_layout(user_id, &db, "channel", page).await.into_response()
}


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(page))
}

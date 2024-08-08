use axum::{Router, extract::{State, Path}, response::{IntoResponse, Html}, routing::get};
use hyper::HeaderMap;
use sailfish::TemplateOnce;
use sqlx::PgPool;
use uuid::Uuid;
use crate::{AppState, logged_user::LoggedUser};


#[derive(TemplateOnce, Default)]
#[template(path = "flow/main.html")]
struct FlowPage {
    id: String,
    name: String,
    data: String
}

async fn flow_page
    (LoggedUser(user_id): 
    LoggedUser, State(db): State<PgPool>, 
    Path(id) : Path<Uuid> 
) -> impl IntoResponse {
    let flow = sqlx::query!(r#"select f.* , e.data as data from flow f
        join flow_editor e on f.id = e.id
        where f.id = $1 and f.user_id = $2"#, 
        id,
        user_id
    )
        .fetch_one(&db)
        .await;

    match flow {
        Err(_) => {
            let mut headers = HeaderMap::new();
            headers.insert("location", "/404.html".parse().unwrap());
            headers.into_response()
        },
        Ok(flow) => {
            let page = FlowPage {
                id: id.to_string(),
                name: flow.name,
                data: flow.data.to_string()
            };
        
            Html(page.render_once().unwrap()).into_response()
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:flow_id", get(flow_page))
}

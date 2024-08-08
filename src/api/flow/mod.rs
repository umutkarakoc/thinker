use axum::{Router, extract::{State, Path}, response::IntoResponse, 
    routing::post, Json};
use hyper::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{AppState, logged_user::LoggedUser};

mod list;
mod step_list;
mod send_message;
mod wait_for_reply;

async fn update_editor_data
    (LoggedUser(user_id): 
    LoggedUser, State(db): State<PgPool>, 
    Path(id) : Path<Uuid> ,
    Json(data): Json<Value>
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
            (StatusCode::FORBIDDEN).into_response()
        },
        Ok(_flow) => {
            sqlx::query!("update flow_editor set data = $2 where id = $1",
                id,
                data)
            .execute(&db)
            .await
            .unwrap();

            Json(json!({"ok": true})).into_response()
        }
    }
}

#[derive(Deserialize)]
struct ConnectStepParams {
    from: Uuid,
    to: Option<Uuid>,
    flow_id: Uuid
}
async fn connect_step
    (LoggedUser(user_id): 
    LoggedUser, State(db): State<PgPool>, 
    Json(params): Json<ConnectStepParams>
) -> impl IntoResponse {
    let flow = sqlx::query!(r#"select f.* , e.data as data from flow f
        join flow_editor e on f.id = e.id
        where f.id = $1 and f.user_id = $2"#, 
        params.flow_id,
        user_id
    )
        .fetch_one(&db)
        .await;

    match flow {
        Err(_) => {
            (StatusCode::FORBIDDEN).into_response()
        },
        Ok(_flow) => {
            if params.to.is_some() {
                sqlx::query!(r#"insert into flow_connection ("id", "to") 
                    values ($1, $2) 
                    on conflict ("id") do update set "to" = $2 "#,
                    params.from,
                    params.to.unwrap())
                .execute(&db)
                .await
                .unwrap();
            } else {
                sqlx::query!(r#"delete from flow_connection where "id" = $1"#,
                    params.from)
                .execute(&db)
                .await
                .unwrap();
            }

            Json(json!({"ok": true})).into_response()
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/step_list", post(step_list::handler))
        .route("/update_editor_data/:flow_id", post(update_editor_data))
        .route("/connect_step", post(connect_step))
        .merge(wait_for_reply::router())
        .merge(send_message::router())
        .merge(list::router())
}

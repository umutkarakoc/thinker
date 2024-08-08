use crate::models::User;
use crate::AppState;
use async_session::{CookieStore, Session, SessionStore};
use axum::extract::State;
use axum::headers::HeaderMap;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Form, Router};
use bcrypt::verify;
use hyper::header::SET_COOKIE;
use sailfish::TemplateOnce;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(TemplateOnce)]
#[template(path = "admin/login.html")]
pub struct LoginTemplate {
    pub email: String,
    pub error: String
}

async fn get_login() -> impl IntoResponse {
    let t = LoginTemplate {
        email: "".to_string(),
        error: "".to_string()
    };

    Html(t.render_once().unwrap()).into_response()
}

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

async fn post_login(
    State(store): State<CookieStore>,
    State(db): State<PgPool>,
    Form(params): Form<LoginParams>,
) -> impl IntoResponse {
    let user = sqlx::query_as!(
        User,
        r#"select * from "admin" where email = $1 and deleted = false "#,
        params.email
    )
    .fetch_one(&db)
    .await;

    let verified = user
        .map(|user| {
            let verified = verify(params.password, &user.password).unwrap_or(false);
            if verified {
                Some(user)
            } else {
                None
            }
        })
        .unwrap_or(None);

    match verified {
        Some(user) => {
            let mut session = Session::new();
            session.insert("user_id", user.id).unwrap();
            let token = store.store_session(session).await.unwrap().unwrap();

            let cookie = format!(
                "thinker_admin_token={}; HttpOnly; Max-Age={}; SameSite=None; Secure; Path=/",
                token,
                60 * 60 * 24 * 30 * 5
            );

            let mut headers = HeaderMap::new();
            headers.insert(SET_COOKIE, cookie.parse().unwrap());
            headers.insert("location", "/admin".parse().unwrap());

            (StatusCode::FOUND, headers).into_response()
        }
        None => {
            let t = LoginTemplate {
                email: params.email,
                error: "Invalid credentials".to_string(),
            };

            Html(t.render_once().unwrap()).into_response()
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/login", get(get_login).post(post_login))
}

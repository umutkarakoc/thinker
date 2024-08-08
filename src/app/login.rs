use crate::models::User;
use crate::AppState;
use axum::extract::State;
use axum::headers::HeaderMap;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Html};
use axum::routing::get;
use axum::{Router, Form};
use async_session::{Session, SessionStore as _, CookieStore};
use bcrypt::verify;
use hyper::header::{SET_COOKIE, LOCATION};
use sailfish::TemplateOnce;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(TemplateOnce, Default)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

async fn page() -> impl IntoResponse {
    Html(LoginTemplate::default().render_once().unwrap())
}

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

async fn login(
    State(store): State<CookieStore>,
    State(db): State<PgPool>,
    Form(params): Form<LoginParams>,
) -> impl IntoResponse {
    let user = sqlx::query_as!(
        User,
        r#"select * from "user" where email = $1 and deleted = false "#,
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
                "thinker_token={}; HttpOnly; Max-Age={}; SameSite=None; Secure; Path=/",
                token,
                60 * 60 * 24 * 30 * 5
            );

            let mut headers = HeaderMap::new();
            headers.insert(SET_COOKIE, cookie.parse().unwrap());
            headers.insert("hx-redirect", "/app".parse().unwrap());

            (StatusCode::FOUND, headers).into_response()
        }
        None => {
            ("Invalid credentials").into_response()
        }
    }
}

async fn logout() -> impl IntoResponse {
    let cookie = format!("thinker_token=deleted; HttpOnly; SameSite=None; Secure; Path=/");

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());
    headers.insert(LOCATION, "/login".parse().unwrap());
    (headers, StatusCode::FOUND).into_response()
}

pub fn router() -> Router<AppState> {
    Router::new().route("/login", get(page).post(login))
        .route("/logout", get(logout).post(logout))
}

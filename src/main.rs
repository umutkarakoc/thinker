extern crate bcrypt;
extern crate hyper;
extern crate hyper_native_tls;
mod app;
mod appconfig;
mod logged_user;
mod models;
mod service;
mod storage;
// mod admin;
mod api;
mod task;
use crate::appconfig::ENV;
use async_session::CookieStore;
use axum::extract::FromRef;
use axum::{
    body::Body,
    http::{HeaderValue, StatusCode},
    response::IntoResponse,
    routing::get_service, Router,
};
use hyper::{client::HttpConnector, Method};
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::{io, str::FromStr};
use tokio::sync::OnceCell;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type HyperClient = hyper::client::Client<HttpConnector, Body>;

pub static REQWEST: Lazy<reqwest::Client> =
    Lazy::new(|| reqwest::Client::builder().build().unwrap());
pub static HYPER_CLIENT: Lazy<HyperClient> = Lazy::new(|| HyperClient::new());
pub static DB: OnceCell<PgPool> = OnceCell::const_new();

async fn init_db() -> PgPool {
    PgPoolOptions::new()
        .max_connections(30)
        .connect(&ENV.database_url)
        .await
        .expect("cant connect to database")
}

pub async fn get_db() -> PgPool {
    DB.get_or_init(init_db).await.clone()
}

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    session_store: CookieStore,
}
impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.db.clone()
    }
}
impl FromRef<AppState> for CookieStore {
    fn from_ref(app_state: &AppState) -> CookieStore {
        app_state.session_store.clone()
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_str(ENV.rust_log.as_str()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = get_db().await;

    let _secret = &ENV.secret.clone().into_bytes()[..];
    let session_store = CookieStore::new();

    task::init(db.clone());

    let app_state = AppState { db, session_store };

    let serve_dir =
        get_service(ServeDir::new(ENV.assets.clone()).append_index_html_on_directories(true))
            .handle_error(handle_error);

    let app = Router::new()
        // .route("/", any(v3))
        // .route("/:page", any(v3))
        // .route("/api/:service/:action", any(v3))
        .nest("/api", api::router())
        .nest("/app", app::router())
        .nest("/storage", storage::router())
        .with_state(app_state)
        // .nest("/admin", admin::router())
        .nest_service("/", serve_dir)
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ]),
        )
        .layer(TraceLayer::new_for_http());

    axum_server::bind(ENV.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

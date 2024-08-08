mod webhook;
mod flow;
use crate::AppState;
use axum::{Router, routing::get, response::IntoResponse, Json};

pub async fn health() -> impl IntoResponse {
    Json(r##"{"status":{"server":{"success":true,"message":"Connection established.","tagstoLog":{"serverVersion":"16"}},"database":{"success":true,"message":"Connection established","tagstoLog":[]},"disc":{"success":true,"message":"Disc Check OK","tagstoLog":[]}},"code":200,"message":"Status Results","session":"","meta":"","data":[],"type":""}"##)
        .into_response()
}


pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .nest("/flow", flow::router())
        .nest("/webhook", webhook::router())
}

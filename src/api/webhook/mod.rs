mod whatsapp;
mod instagram;
use crate::{AppState, models::Message};
use axum::{
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    routing::post,
    Router, Json,
};
use hyper::StatusCode;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::Value;
use sqlx::{PgPool};
use whatsapp::*;


pub static REQWEST: Lazy<reqwest::Client> =
    Lazy::new(|| reqwest::Client::builder().build().unwrap());
static FB_TOKEN: &str = "wnSMSVs37KvCXZwnKLaRY8f7wLf5S5nZMMR4sLZtWwzz86pdB3d3n2sPLvaZ";

#[derive(Deserialize)]
pub struct FBToken {
    #[serde(rename(deserialize = "hub.mode"))]
    mode: Option<String>,
    #[serde(rename(deserialize = "hub.verify_token"))]
    verify_token: Option<String>,
    #[serde(rename(deserialize = "hub.challenge"))]
    challenge: Option<String>,
}
pub async fn fb_webhook_challenge(Query(query): Query<FBToken>) -> impl IntoResponse {
    if let FBToken {
        mode: Some(mode),
        verify_token: Some(token),
        challenge: Some(challenge),
    } = query
    {
        if mode.eq("subscribe") && token.eq(FB_TOKEN) {
            challenge.clone().into_response()
        } else {
            StatusCode::FORBIDDEN.into_response()
        }
    } else {
        StatusCode::BAD_REQUEST.into_response()
    }
}


pub async fn fb_webhook(
    State(db): State<PgPool>,
    Path(t): Path<String>,
    Json(params): Json<Value>,
) -> impl IntoResponse {
    
    let uri = format!("https://webhook.helorobo.com/webhook/{}", t);

    let res = REQWEST
        .post(uri)
        .header("content-type", "application/json")
        .body(params.to_string())
        .send()
        .await;


    let uri = format!("http://127.0.0.1:3000/api/webhook/{}", t); //sonitel

    let _res = REQWEST
        .post(uri)
        .header("content-type", "application/json")
        .body(params.to_string())
        .send()
        .await;

    instagram::instagram(&db, &params).await;


    if let Ok(result) = res {
        Response::builder()
            .header("content-type", "application/json")
            .body(result.text().await.unwrap())
            .unwrap()
            .into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}

pub async fn fb_webhook_test(
    Path(t): Path<String>,
    Json(params): Json<Value>,
) -> impl IntoResponse {

    let uri = format!("https://webhook.test.helorobo.net/webhook/{}", t);

    let res = REQWEST
        .post(uri)
        .header("content-type", "application/json")
        .body(params.to_string())
        .send()
        .await;

    if let Ok(result) = res {
        Response::builder()
            .header("content-type", "application/json")
            .body(result.text().await.unwrap())
            .unwrap()
            .into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/test/:t", post(fb_webhook_test).get(fb_webhook_challenge))
        .route("/:t", post(fb_webhook).get(fb_webhook_challenge))
        .route("/whatsapp/:id", post(whatsapp))
}

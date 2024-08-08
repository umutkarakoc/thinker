use std::default;

use crate::logged_user::LoggedAdmin;
use crate::models::User;
use crate::AppState;
use async_session::{CookieStore, Session, SessionStore};
use axum::extract::{State, Path};
use axum::headers::HeaderMap;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Form, Router};
use bcrypt::{verify, hash};
use regex::Regex;
use sailfish::TemplateOnce;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::Layout;

#[derive(TemplateOnce)]
#[template(path = "admin/list_user.html")]
pub struct UserListPage {
    pub users: Vec<User>
}

async fn user_list_page(State(db): State<PgPool>, LoggedAdmin(_) : LoggedAdmin) -> impl IntoResponse {
    let users = sqlx::query_as!(User, r#"select * from "user" "#)
        .fetch_all(&db)
        .await
        .unwrap();

    let page = UserListPage { users };
    let layout = Layout { page };
    Html(layout.render_once().unwrap()).into_response()
}

#[derive(Serialize, Deserialize, Default)]
pub struct CreateUserParams {
    pub email: String,
    pub name: String,
    pub password: String
}

#[derive(TemplateOnce, Default)]
#[template(path = "admin/create_user.html")]
pub struct CreateUserPage {
    pub user: CreateUserParams,
    pub error: String,
}
async fn create_user_page(LoggedAdmin(_) : LoggedAdmin) -> impl IntoResponse {
    let page = CreateUserPage::default();
    let layout = Layout { page };
    Html(layout.render_once().unwrap()).into_response()
}


async fn create_user(
    State(db): State<PgPool>, 
    LoggedAdmin(_) : LoggedAdmin, 
    Form(params) : Form<CreateUserParams>) -> impl IntoResponse {

    let password = hash(params.password.clone(), 10).unwrap();
    let result = sqlx::query!(r#"insert into "user" (email, name, password)
    values ($1, $2, $3) "#, params.email, params.name, password)
    .execute(&db)
    .await;

    let page = CreateUserPage {
        user: CreateUserParams { 
            email: params.email.clone(),
            name: params.name,
            password: params.password, 
        },
        error: "".to_string()
    };

    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    if !email_regex.is_match(&params.email) {
        let page = CreateUserPage{ error: "email is not valid".to_string(), ..page};
        let layout = Layout { page };
        return Html(layout.render_once().unwrap()).into_response()
    }

    match result {
        Ok(_) => {
            let mut headers = HeaderMap::new();
            headers.insert("location", "/admin/user".parse().unwrap());
            (StatusCode::FOUND, headers).into_response()
        },
        Err(_) => {
            let page = CreateUserPage{ error: "This email is in use".to_string(), ..page};
            let layout = Layout { page };
            Html(layout.render_once().unwrap()).into_response()
        }
    }
}

#[derive(TemplateOnce, Default)]
#[template(path = "admin/edit_user.html")]
pub struct EditUserPage {
    pub user: User,
    pub error: String,
}
async fn user_page(State(db): State<PgPool>, LoggedAdmin(_) : LoggedAdmin, Path(id) : Path<Uuid>) -> impl IntoResponse {
    let user = sqlx::query_as!(User, r#"select * from "user" where id = $1"# , id)
        .fetch_one(&db).await.unwrap();

    let user = User {password: "".to_string(), ..user};
    let page = EditUserPage {
        user,
        error: "".to_string()
    };
    let layout = Layout { page };

    Html(layout.render_once().unwrap()).into_response()
}

async fn update_user(
    State(db): State<PgPool>, 
    LoggedAdmin(_) : LoggedAdmin, 
    Form(user) : Form<User>) -> impl IntoResponse {

    let password = hash(user.password.clone(), 10).unwrap();
    let result = sqlx::query!(r#"update "user" set email =  $1, name = $2"#, user.email, user.name)
    .execute(&db)
    .await;


    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    if !email_regex.is_match(&user.email) {
        let page = EditUserPage{ error: "email is not valid".to_string(), user};
        let layout = Layout { page };
        return Html(layout.render_once().unwrap()).into_response()
    }

    match result {
        Ok(_) => {
            let mut headers = HeaderMap::new();
            headers.insert("location", "/admin/user".parse().unwrap());
            (StatusCode::FOUND, headers).into_response()
        },
        Err(_) => {
            let page = EditUserPage{ error: "This email is in use".to_string(), user};
            let layout = Layout { page };
            Html(layout.render_once().unwrap()).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/user", get(user_list_page))
        .route("/user/create", get(create_user_page).post(create_user))
        .route("/user/:id", get(user_page).post(update_user))
}

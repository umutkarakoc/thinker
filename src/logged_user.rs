use async_session::{CookieStore, SessionStore as _};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, TypedHeader},
    headers::Cookie,
    http::request::Parts,
    response::Redirect,
    RequestPartsExt,
};
use uuid::Uuid;

pub struct LoggedUser(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for LoggedUser
where
    CookieStore: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = CookieStore::from_ref(state);
        let cookie: Option<TypedHeader<Cookie>> = parts.extract().await.unwrap();
        let token = cookie
            .as_ref()
            .and_then(|cookie| cookie.get("thinker_token"));

        match token {
            Some(token) => {
                let session = store.load_session(token.to_string()).await;
                match session {
                    Ok(Some(session)) => {
                        let id: Uuid = session.get("user_id").unwrap();
                        Ok(LoggedUser(id))
                    }
                    _ => Err(Redirect::to("/app/login")),
                }
            }
            None => Err(Redirect::to("/app/login")),
        }
    }
}

pub struct LoggedAdmin(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for LoggedAdmin
where
    CookieStore: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Redirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = CookieStore::from_ref(state);
        let cookie: Option<TypedHeader<Cookie>> = parts.extract().await.unwrap();
        let token = cookie
            .as_ref()
            .and_then(|cookie| cookie.get("thinker_admin_token"));

        match token {
            Some(token) => {
                let session = store.load_session(token.to_string()).await;
                match session {
                    Ok(Some(session)) => {
                        let id: Uuid = session.get("user_id").unwrap();
                        Ok(LoggedAdmin(id))
                    }
                    _ => Err(Redirect::to("/admin/login")),
                }
            }
            None => Err(Redirect::to("/admin/login")),
        }
    }
}

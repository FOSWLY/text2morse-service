use crate::{
    data::config::CONFIG,
    templates::HtmlTemplate,
    utils::session::{self, AuthPayload, create_session},
};
use askama::Template;
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use moka::future::Cache;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use time::Duration;

pub async fn get_auth_handle() -> impl IntoResponse {
    let sys_time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(timestamp) => timestamp.as_millis(),
        Err(_) => 0,
    };

    let url = format!(
        "https://oauth.yandex.ru/authorize?client_id={0}&response_type=token&widget_kind=button-stub&et={1}&force_confirm=1",
        &CONFIG.client_id, &sys_time
    );
    Redirect::to(&url)
}

#[derive(Template)]
#[template(path = "callback.html")]
struct CallbackTemplate {}

pub async fn get_auth_callback() -> impl IntoResponse {
    let template = CallbackTemplate {};
    let html_template = HtmlTemplate(template);
    html_template.into_response()
}

#[derive(Template)]
#[template(path = "logout.html")]
struct LogoutTemplate {}

pub async fn get_auth_logout(jar: CookieJar) -> impl IntoResponse {
    let is_authenticated = session::decrypt_session(&jar).is_ok();
    if !is_authenticated {
        return Redirect::to("/").into_response();
    }

    let template = LogoutTemplate {};
    let html_template = HtmlTemplate(template);
    html_template.into_response()
}

#[derive(Serialize)]
pub struct AuthSuccess {
    pub status: String,
}

#[derive(Serialize)]
pub struct AuthError {
    pub error: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum AuthResponse {
    Success(AuthSuccess),
    Error(AuthError),
}

pub async fn post_auth_login(
    jar: CookieJar,
    State(cache): State<Cache<String, String>>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    let session = match create_session(cache, payload).await {
        Ok(session) => session,
        Err(err) => {
            let response = Json(AuthResponse::Error(AuthError { error: err }));
            return (jar, (StatusCode::BAD_REQUEST, response));
        }
    };

    let cookie = Cookie::build(("t2mc_session", session))
        .path("/")
        .http_only(true)
        .domain(&CONFIG.cookie_domain)
        .secure(true)
        .max_age(Duration::days(364));
    let response = Json(AuthResponse::Success(AuthSuccess {
        status: "ok".to_string(),
    }));
    (jar.add(cookie), (StatusCode::OK, response))
}

pub async fn delete_auth_logout(jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::build(("t2mc_session", ""))
        .path("/")
        .http_only(true)
        .domain(&CONFIG.cookie_domain)
        .secure(true)
        .max_age(Duration::days(0));
    (jar.add(cookie), StatusCode::NO_CONTENT)
}

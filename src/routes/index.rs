use crate::templates::HtmlTemplate;
use crate::utils::session;

use askama::Template;
use axum::response::IntoResponse;
use axum_extra::extract::cookie::CookieJar;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    is_authenticated: bool,
}

pub async fn get_index(jar: CookieJar) -> impl IntoResponse {
    let is_authenticated = session::decrypt_session(&jar).is_ok();
    let template = IndexTemplate { is_authenticated };
    let html_template = HtmlTemplate(template);
    let response = html_template.into_response();
    response
}

pub async fn get_robots_txt() -> impl IntoResponse {
    "User-agent: *\nDisallow: /\nAllow: /$\nAllow: /legal/\n"
}

use crate::{templates::HtmlTemplate, utils::session};

use askama::Template;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

#[derive(Template)]
#[template(path = "profile.html")]
struct ProfileTemplate {
    username: String,
    avatar_id: String,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    error: String,
}

pub async fn get_my_profile(jar: CookieJar) -> impl IntoResponse {
    let session = match session::decrypt_session(&jar) {
        Ok(session) => session,
        Err(_) => {
            let template = ErrorTemplate {
                error: "Unauthorized".to_string(),
            };
            let html_template = HtmlTemplate(template);
            return html_template.into_response();
        }
    };

    let template = ProfileTemplate {
        username: session.username,
        avatar_id: session.avatar_id,
    };
    let html_template = HtmlTemplate(template);
    html_template.into_response()
}

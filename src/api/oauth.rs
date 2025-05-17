use axum::http::{HeaderMap, HeaderValue};
use lazy_static::lazy_static;
use reqwest::{Client, Error, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref REQ_CLIENT: Client = Client::new();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct YandexInfoResponse {
    pub id: String,
    pub login: String,
    pub default_avatar_id: String,
}

pub async fn get_info(oauth_token: &String) -> Result<YandexInfoResponse, Error> {
    let mut headers = HeaderMap::new();
    let oauth_token = format!("OAuth {oauth_token}");
    if let Ok(val) = HeaderValue::from_str(&oauth_token) {
        headers.insert(AUTHORIZATION, val);
    }

    let data = REQ_CLIENT
        .get("https://login.yandex.ru/info?format=json")
        .headers(headers)
        .send()
        .await?
        .json::<YandexInfoResponse>()
        .await?;

    Ok(data)
}

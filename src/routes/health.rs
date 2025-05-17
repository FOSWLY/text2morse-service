use axum::{response::IntoResponse, Json};
use serde::Serialize;

use crate::data::config::CONFIG;

#[derive(Serialize)]
pub struct Health {
    version: String,
    status: String,
}

pub async fn get_health() -> impl IntoResponse {
    let data = Health {
        version: CONFIG.version.clone(),
        status: "ok".to_string(),
    };

    Json(data)
}

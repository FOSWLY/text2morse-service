use crate::{
    api::oauth::{self},
    data::config::CONFIG,
};
use axum_extra::extract::CookieJar;
use base64::{Engine as _, prelude::BASE64_STANDARD};
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, AeadCore, Key, KeyInit, OsRng},
};
use lazy_static::lazy_static;
use moka::future::Cache;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref CIPHER: XChaCha20Poly1305 = {
        let key = Key::<XChaCha20Poly1305>::from_slice(&CONFIG.session_key.as_bytes());
        XChaCha20Poly1305::new(&key)
    };
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AuthPayload {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SessionData {
    pub access_token: String,
    pub expires_in: String,
    pub avatar_id: String,
    pub username: String,
}

async fn get_session_data(
    cache: Cache<String, String>,
    payload: &AuthPayload,
) -> Result<SessionData, String> {
    if let Err(_) = payload.expires_in.parse::<u64>() {
        return Err("Invalid expires_in value".to_string());
    }

    if payload.token_type != "bearer" {
        return Err("Invalid token_type value".to_string());
    }

    let cache_key = format!("oauth:token:{}", &payload.access_token);
    if let Some(cached) = cache.get(&cache_key).await {
        let session_data = serde_json::from_str(&cached).unwrap();
        return Ok(session_data);
    }

    let user = oauth::get_info(&payload.access_token)
        .await
        .map_err(|_| "Failed to get user info".to_string())?;

    let session_data = SessionData {
        access_token: payload.access_token.clone(),
        expires_in: payload.expires_in.clone(),
        avatar_id: user.default_avatar_id.clone(),
        username: user.login.clone(),
    };

    let cache_body = serde_json::to_string(&session_data).unwrap();
    cache.insert(cache_key, cache_body).await;
    Ok(session_data)
}

pub async fn create_session(
    cache: Cache<String, String>,
    payload: AuthPayload,
) -> Result<String, String> {
    let session_data = match get_session_data(cache, &payload).await {
        Ok(session_data) => session_data,
        Err(err) => return Err(err),
    };

    let data = match serde_json::to_string(&session_data) {
        Ok(data) => data,
        Err(_) => return Err("Failed to serialize payload".to_string()),
    };

    let session = match encrypt_data(data.as_bytes()) {
        Ok(session) => session,
        Err(_) => return Err("Failed to encrypt data".to_string()),
    };

    Ok(session)
}

fn encrypt_data(payload: &[u8]) -> Result<String, chacha20poly1305::Error> {
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = CIPHER.encrypt(&nonce, payload.as_ref())?;
    let encoded_text = BASE64_STANDARD.encode(&ciphertext);
    let encoded_nonce = BASE64_STANDARD.encode(&nonce);
    let encoded_session = format!("{encoded_nonce}:{encoded_text}");
    Ok(encoded_session)
}

fn decrypt_data(ciphertext: &[u8], nonce_value: &[u8]) -> Result<String, String> {
    let nonce = XNonce::from_slice(nonce_value);
    let vec = match CIPHER.decrypt(&nonce, ciphertext) {
        Ok(vec) => vec,
        Err(_) => return Err("Invalid session".to_string()),
    };

    match String::from_utf8(vec) {
        Ok(data) => Ok(data),
        Err(_) => Err("Invalid session".to_string()),
    }
}

pub fn decrypt_session(jar: &CookieJar) -> Result<SessionData, String> {
    let session = jar
        .get("t2mc_session")
        .map(|cookie| cookie.value().to_owned())
        .ok_or("Session not found")?;

    let (encoded_nonce, encoded_session) =
        session.split_once(':').ok_or("Invalid session format")?;

    let nonce_vec = BASE64_STANDARD
        .decode(encoded_nonce)
        .map_err(|_| "Invalid nonce")?;
    let session_vec = BASE64_STANDARD
        .decode(encoded_session)
        .map_err(|_| "Invalid session")?;
    let data = decrypt_data(&session_vec, &nonce_vec)?;
    let auth = serde_json::from_str::<SessionData>(&data)
        .map_err(|_| "Failed to deserialize session data".to_string())?;

    Ok(auth)
}

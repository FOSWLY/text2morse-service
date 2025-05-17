use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, time::Duration};

#[derive(Deserialize)]
pub struct CargoConfig {
    package: CargoPackage,
}

#[derive(Deserialize)]
pub struct CargoPackage {
    version: String,
}

pub struct Config {
    pub version: String,
    pub hostname: String,
    pub port: u16,
    pub cache_ttl: Duration,
    pub client_id: String,
    pub session_key: String,
    pub cookie_domain: String,
}

lazy_static! {
    static ref CARGO_CONFIG: CargoConfig =
        toml::from_str(include_str!("../../Cargo.toml")).unwrap();
    pub static ref CONFIG: Config = Config {
        version: CARGO_CONFIG.package.version.clone(),
        hostname: match env::var("SERVICE_HOST") {
            Ok(host) => host,
            Err(_) => "127.0.0.1".to_string(),
        },
        port: match env::var("SERVICE_PORT") {
            Ok(port) => port.parse().unwrap(),
            Err(_) => 7674,
        },
        cache_ttl: Duration::from_secs(7200),
        client_id: match env::var("CLIENT_ID") {
            Ok(client_id) => client_id,
            Err(_) => panic!("CLIENT_ID environment variable not set"),
        },
        session_key: match env::var("SESSION_KEY") {
            Ok(session_key) => {
                if session_key.len() != 32 {
                    panic!("SESSION_KEY must be 32 bytes long");
                }
                session_key
            }
            Err(_) => panic!("SESSION_KEY environment variable not set"),
        },
        cookie_domain: match env::var("COOKIE_DOMAIN") {
            Ok(cookie_domain) => cookie_domain,
            Err(_) => panic!("COOKIE_DOMAIN environment variable not set"),
        },
    };
}

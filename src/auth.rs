use base64::Engine;
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::config::{Config, ServiceType};
use crate::crypto::hmac_sha256_base64;
use crate::error::{JknError, Result};

pub struct SignedHeaders {
    pub headers: HeaderMap,
    pub timestamp: String,
}

pub fn build_headers(config: &Config, service: ServiceType) -> Result<SignedHeaders> {
    let timestamp = Utc::now().timestamp().to_string();
    let message = format!("{}&{}", config.cons_id, timestamp);
    let signature = hmac_sha256_base64(config.cons_secret.as_bytes(), message.as_bytes())?;
    let user_key = config.user_key(service)?;
    let pcare_auth =
        base64::engine::general_purpose::STANDARD.encode(format!("{}:095", config.pcare_user_pass));

    let mut headers = HeaderMap::new();
    insert(&mut headers, "X-cons-id", &config.cons_id)?;
    insert(&mut headers, "X-timestamp", &timestamp)?;
    insert(&mut headers, "X-signature", &signature)?;
    insert(&mut headers, "user_key", user_key)?;
    insert(
        &mut headers,
        "X-Authorization",
        &format!("Basic {}", pcare_auth),
    )?;

    Ok(SignedHeaders { headers, timestamp })
}

fn insert(headers: &mut HeaderMap, key: &'static str, value: &str) -> Result<()> {
    let name = HeaderName::from_bytes(key.to_ascii_lowercase().as_bytes())
        .map_err(|err| JknError::Config(format!("invalid header {key}: {err}")))?;
    let value = HeaderValue::from_str(value)
        .map_err(|err| JknError::Config(format!("invalid header {key}: {err}")))?;
    headers.insert(name, value);
    Ok(())
}

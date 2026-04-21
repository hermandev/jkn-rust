use std::sync::Arc;

use reqwest::Method;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::auth::build_headers;
use crate::config::{Config, ServiceType};
use crate::crypto::{decrypt_response_payload, parse_html};
use crate::error::{JknError, Result};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    pub code: Value,
    pub message: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JknResponse {
    #[serde(default)]
    pub response: Value,
    #[serde(rename = "metaData", default)]
    pub meta_data: Option<Metadata>,
    #[serde(default)]
    pub metadata: Option<Metadata>,
}

impl Metadata {
    pub fn code_as_string(&self) -> String {
        match &self.code {
            Value::String(value) => value.clone(),
            other => other.to_string(),
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self.code_as_string().as_str(), "200")
    }
}

impl JknResponse {
    pub fn metadata(&self) -> Option<&Metadata> {
        self.meta_data.as_ref().or(self.metadata.as_ref())
    }

    pub fn is_success(&self) -> bool {
        self.metadata().is_some_and(Metadata::is_success)
    }

    pub fn response_as<T: DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_value(self.response.clone()).map_err(JknError::from)
    }

    pub fn into_response<T: DeserializeOwned>(self) -> Result<T> {
        serde_json::from_value(self.response).map_err(JknError::from)
    }
}

#[derive(Debug, Clone)]
pub struct RequestOptions {
    pub path: String,
    pub method: Method,
    pub data: Option<Value>,
    pub skip_decrypt: bool,
    pub skip_content_type_hack: bool,
    pub headers: HeaderMap,
}

impl RequestOptions {
    pub fn get(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            method: Method::GET,
            data: None,
            skip_decrypt: false,
            skip_content_type_hack: false,
            headers: HeaderMap::new(),
        }
    }

    pub fn post(path: impl Into<String>) -> Self {
        Self {
            method: Method::POST,
            ..Self::get(path)
        }
    }

    pub fn put(path: impl Into<String>) -> Self {
        Self {
            method: Method::PUT,
            ..Self::get(path)
        }
    }

    pub fn delete(path: impl Into<String>) -> Self {
        Self {
            method: Method::DELETE,
            ..Self::get(path)
        }
    }

    pub fn data<T: Serialize>(mut self, data: T) -> Result<Self> {
        self.data = Some(serde_json::to_value(data)?);
        Ok(self)
    }

    pub fn skip_decrypt(mut self) -> Self {
        self.skip_decrypt = true;
        self
    }

    pub fn skip_content_type_hack(mut self) -> Self {
        self.skip_content_type_hack = true;
        self
    }

    pub fn header(mut self, key: &'static str, value: &str) -> Result<Self> {
        let name = HeaderName::from_bytes(key.as_bytes())
            .map_err(|err| JknError::Config(format!("invalid header {key}: {err}")))?;
        let value = HeaderValue::from_str(value)
            .map_err(|err| JknError::Config(format!("invalid header {key}: {err}")))?;
        self.headers.insert(name, value);
        Ok(self)
    }
}

#[derive(Clone)]
pub struct JknClient {
    http: reqwest::Client,
    config: Arc<Config>,
}

impl JknClient {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self {
            http: reqwest::Client::new(),
            config: Arc::new(config.validate()?),
        })
    }

    pub fn from_env() -> Result<Self> {
        Self::new(Config::from_env()?)
    }

    pub fn config(&self) -> &Config {
        self.config.as_ref()
    }

    pub async fn send(&self, service: ServiceType, options: RequestOptions) -> Result<JknResponse> {
        let signed = build_headers(self.config(), service)?;
        let base_url = self.config().base_url(service)?;
        let url = format!("{}{}", base_url, options.path);

        let mut headers = signed.headers;
        for (name, value) in &options.headers {
            headers.insert(name.clone(), value.clone());
        }

        let mut request = self
            .http
            .request(options.method.clone(), &url)
            .headers(headers);

        if let Some(data) = options.data {
            if options.method == Method::GET {
                return Err(JknError::InvalidArgument(
                    "can not pass data with GET method".to_string(),
                ));
            }

            request = request.body(serde_json::to_string(&data)?);

            if !options.skip_content_type_hack && !options.headers.contains_key(CONTENT_TYPE) {
                request = request.header(CONTENT_TYPE, "Application/x-www-form-urlencoded");
            }
        }

        let response = request.send().await?;
        let status = response.status();
        let body = response.text().await?;
        if body.is_empty() {
            return Err(JknError::InvalidArgument(format!(
                "the response body is empty ({status})"
            )));
        }

        let mut parsed: JknResponse = serde_json::from_str(&body).map_err(|err| {
            JknError::Json(serde_json::Error::io(std::io::Error::other(format!(
                "the response is not JSON ({}) | source: {err}",
                parse_html(&body)
            ))))
        })?;

        if !options.skip_decrypt
            && parsed.response != Value::Null
            && parsed.response != Value::String(String::new())
        {
            if let Some(encrypted) = parsed.response.as_str() {
                let decrypted = decrypt_response_payload(
                    encrypted,
                    &self.config().cons_id,
                    &self.config().cons_secret,
                    &signed.timestamp,
                )?;
                parsed.response = serde_json::from_str(&decrypted)?;
            }
        }

        Ok(parsed)
    }

    pub async fn send_typed<T: DeserializeOwned>(
        &self,
        service: ServiceType,
        options: RequestOptions,
    ) -> Result<T> {
        self.send(service, options).await?.into_response()
    }
}

pub fn normalize_path(path: &str, params: &[(&str, Option<String>)]) -> Result<String> {
    if !path.starts_with('/') {
        return Err(JknError::Path("path must start with '/'".to_string()));
    }

    let mut output = String::with_capacity(path.len());
    let bytes = path.as_bytes();
    let mut idx = 0;

    while idx < bytes.len() {
        if bytes[idx] == b'/' && idx + 1 < bytes.len() && bytes[idx + 1] == b':' {
            let start = idx + 2;
            let mut end = start;
            while end < bytes.len() {
                let ch = bytes[end] as char;
                if ch.is_ascii_alphanumeric() || ch == '_' {
                    end += 1;
                } else {
                    break;
                }
            }

            let optional = end < bytes.len() && bytes[end] == b'?';
            let key = &path[start..end];
            let value = params
                .iter()
                .find(|(name, _)| *name == key)
                .and_then(|(_, value)| value.clone());

            if let Some(value) = value {
                output.push('/');
                output.push_str(&encode_path_component(&value));
            } else if !optional {
                return Err(JknError::Path(format!("missing params: {key}")));
            }

            idx = end + usize::from(optional);
            continue;
        }

        output.push(bytes[idx] as char);
        idx += 1;
    }

    Ok(output)
}

fn encode_path_component(value: &str) -> String {
    let already_encoded = urlencoding::decode(value)
        .map(|decoded| urlencoding::encode(&decoded).as_ref() == value)
        .unwrap_or(false);
    if already_encoded {
        value.to_string()
    } else {
        urlencoding::encode(value).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn path_params_behave_like_ts_version() {
        assert_eq!(
            normalize_path(
                "/users/:id/:group?",
                &[("id", Some("123".into())), ("group", None)]
            )
            .expect("path should normalize"),
            "/users/123"
        );
        assert_eq!(
            normalize_path("/search/:q", &[("q", Some("hello world".into()))])
                .expect("path should normalize"),
            "/search/hello%20world"
        );
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct DemoPayload {
        value: String,
    }

    #[test]
    fn typed_response_helpers_work() {
        let response = JknResponse {
            response: json!({ "value": "ok" }),
            meta_data: Some(Metadata {
                code: json!("200"),
                message: "OK".into(),
                extra: Map::new(),
            }),
            metadata: None,
        };

        assert!(response.is_success());
        assert_eq!(
            response
                .response_as::<DemoPayload>()
                .expect("payload should deserialize"),
            DemoPayload { value: "ok".into() }
        );
    }
}

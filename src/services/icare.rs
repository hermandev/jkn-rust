use std::sync::Arc;

use serde::Serialize;

use crate::JknClient;
use crate::client::{JknResponse, RequestOptions};
use crate::config::ServiceType;
use crate::error::Result;

#[derive(Clone)]
pub struct ICare {
    client: Arc<JknClient>,
}

impl ICare {
    pub fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn fkrtl<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        let request = RequestOptions::post("/api/rs/validate")
            .skip_content_type_hack()
            .header("Content-Type", "Application/json")?
            .data(data)?;
        self.client.send(ServiceType::Icare, request).await
    }

    pub async fn fktp<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        let request = RequestOptions::post("/api/pcare/validate")
            .skip_content_type_hack()
            .header("Content-Type", "Application/json")?
            .data(data)?;
        self.client.send(ServiceType::Icare, request).await
    }
}

use std::sync::Arc;

use serde::Serialize;

use crate::JknClient;
use crate::client::{JknResponse, RequestOptions, normalize_path};
use crate::config::ServiceType;
use crate::error::Result;

#[derive(Clone)]
pub struct AntreanFktp {
    client: Arc<JknClient>,
}

impl AntreanFktp {
    pub fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn ref_poli(&self, tanggal: &str) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Pcare,
                RequestOptions::get(format!("/ref/poli/tanggal/{tanggal}")),
            )
            .await
    }

    pub async fn ref_dokter(&self, tanggal: &str, kode_poli: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/ref/dokter/kodepoli/:kodePoli/tanggal/:tanggal",
            &[
                ("kodePoli", Some(kode_poli.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn tambah<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Pcare,
                RequestOptions::post("/antrean/add").data(data)?,
            )
            .await
    }

    pub async fn update_status<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Pcare,
                RequestOptions::post("/antrean/panggil").data(data)?,
            )
            .await
    }

    pub async fn batal<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Pcare,
                RequestOptions::post("/antrean/batal").data(data)?,
            )
            .await
    }
}

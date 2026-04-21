use std::sync::Arc;

use serde::Serialize;

use crate::JknClient;
use crate::client::{JknResponse, RequestOptions, normalize_path};
use crate::config::ServiceType;
use crate::error::Result;
use crate::models::aplicares::{
    AplicaresBedData, AplicaresDeleteRequest, AplicaresReadResponse, AplicaresRefKamarResponse,
};

#[derive(Clone)]
pub struct Aplicares {
    client: Arc<JknClient>,
}

impl Aplicares {
    pub fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn ref_kamar(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Aplicares,
                RequestOptions::get("/rest/ref/kelas").skip_decrypt(),
            )
            .await
    }

    pub async fn ref_kamar_typed(&self) -> Result<AplicaresRefKamarResponse> {
        self.ref_kamar().await?.into_response()
    }

    pub async fn update<T: Serialize>(
        &self,
        data: T,
        kode_ppk: Option<&str>,
    ) -> Result<JknResponse> {
        let kode_ppk = match kode_ppk {
            Some(value) => value.to_string(),
            None => self.client.config().require_ppk_code()?.to_string(),
        };
        let path = normalize_path("/rest/bed/update/:kodePpk", &[("kodePpk", Some(kode_ppk))])?;
        let request = RequestOptions::post(path)
            .skip_content_type_hack()
            .header("Content-Type", "application/json")?
            .data(data)?;
        self.client.send(ServiceType::Aplicares, request).await
    }

    pub async fn update_bed(
        &self,
        data: AplicaresBedData,
        kode_ppk: Option<&str>,
    ) -> Result<JknResponse> {
        self.update(data, kode_ppk).await
    }

    pub async fn create<T: Serialize>(
        &self,
        data: T,
        kode_ppk: Option<&str>,
    ) -> Result<JknResponse> {
        let kode_ppk = match kode_ppk {
            Some(value) => value.to_string(),
            None => self.client.config().require_ppk_code()?.to_string(),
        };
        let path = normalize_path("/rest/bed/create/:kodePpk", &[("kodePpk", Some(kode_ppk))])?;
        let request = RequestOptions::post(path)
            .skip_content_type_hack()
            .header("Content-Type", "application/json")?
            .data(data)?;
        self.client.send(ServiceType::Aplicares, request).await
    }

    pub async fn create_bed(
        &self,
        data: AplicaresBedData,
        kode_ppk: Option<&str>,
    ) -> Result<JknResponse> {
        self.create(data, kode_ppk).await
    }

    pub async fn read(
        &self,
        start: u32,
        limit: u32,
        kode_ppk: Option<&str>,
    ) -> Result<JknResponse> {
        let kode_ppk = match kode_ppk {
            Some(value) => value.to_string(),
            None => self.client.config().require_ppk_code()?.to_string(),
        };
        let path = normalize_path(
            "/rest/bed/read/:kodePpk/:start/:limit",
            &[
                ("kodePpk", Some(kode_ppk)),
                ("start", Some(start.to_string())),
                ("limit", Some(limit.to_string())),
            ],
        )?;
        self.client
            .send(
                ServiceType::Aplicares,
                RequestOptions::get(path).skip_decrypt(),
            )
            .await
    }

    pub async fn read_typed(
        &self,
        start: u32,
        limit: u32,
        kode_ppk: Option<&str>,
    ) -> Result<AplicaresReadResponse> {
        self.read(start, limit, kode_ppk).await?.into_response()
    }

    pub async fn delete<T: Serialize>(
        &self,
        data: T,
        kode_ppk: Option<&str>,
    ) -> Result<JknResponse> {
        let kode_ppk = match kode_ppk {
            Some(value) => value.to_string(),
            None => self.client.config().require_ppk_code()?.to_string(),
        };
        let path = normalize_path("/rest/bed/delete/:kodePpk", &[("kodePpk", Some(kode_ppk))])?;
        let request = RequestOptions::post(path)
            .skip_content_type_hack()
            .header("Content-Type", "application/json")?
            .data(data)?;
        self.client.send(ServiceType::Aplicares, request).await
    }

    pub async fn delete_bed(
        &self,
        data: AplicaresDeleteRequest,
        kode_ppk: Option<&str>,
    ) -> Result<JknResponse> {
        self.delete(data, kode_ppk).await
    }
}

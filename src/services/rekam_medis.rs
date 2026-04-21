use std::sync::Arc;

use serde::Serialize;
use serde_json::json;

use crate::JknClient;
use crate::client::{JknResponse, RequestOptions};
use crate::config::ServiceType;
use crate::crypto::encrypt_rekam_medis_payload;
use crate::error::Result;

#[derive(Clone)]
pub struct RekamMedis {
    client: Arc<JknClient>,
}

impl RekamMedis {
    pub fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn insert<T: Serialize>(
        &self,
        nomor_sep: &str,
        jenis_pelayanan: &str,
        bulan: u32,
        tahun: u32,
        data_rekam_medis: &T,
    ) -> Result<JknResponse> {
        let plain = serde_json::to_string(data_rekam_medis)?;
        let encrypted = encrypt_rekam_medis_payload(
            &plain,
            &self.client.config().cons_id,
            &self.client.config().cons_secret,
            self.client.config().require_ppk_code()?,
        )?;

        let payload = json!({
            "request": {
                "noSep": nomor_sep,
                "jnsPelayanan": jenis_pelayanan,
                "bulan": bulan.to_string(),
                "tahun": tahun.to_string(),
                "dataMR": encrypted
            }
        });

        let request = RequestOptions::post("/eclaim/rekammedis/insert")
            .skip_content_type_hack()
            .skip_decrypt()
            .header("Content-Type", "text/plain")?
            .data(payload)?;

        self.client.send(ServiceType::RekamMedis, request).await
    }
}

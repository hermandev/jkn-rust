use std::sync::Arc;

use serde::Serialize;

use crate::JknClient;
use crate::client::{JknResponse, RequestOptions, normalize_path};
use crate::config::ServiceType;
use crate::error::Result;
use crate::models::pcare::{
    PcareAlergiItem, PcareDiagnosaItem, PcareDokterItem, PcareFaskesRujukanSubSpesialisItem,
    PcareKesadaranItem, PcareKhususItem, PcareListResponse, PcarePesertaData, PcarePoliItem,
    PcarePrognosaItem, PcareProviderItem, PcareRiwayatKunjunganItem, PcareRujukanResult,
    PcareSaranaItem, PcareSpesialisItem, PcareStatusPulangItem, PcareSubSpesialisItem,
};
use crate::pcare::PcarePendaftaranResult;

#[derive(Clone)]
pub struct PCare {
    client: Arc<JknClient>,
}

impl PCare {
    pub fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub fn referensi(&self) -> Referensi {
        Referensi::new(Arc::clone(&self.client))
    }

    pub fn peserta(&self) -> Peserta {
        Peserta::new(Arc::clone(&self.client))
    }

    pub fn kunjungan(&self) -> Kunjungan {
        Kunjungan::new(Arc::clone(&self.client))
    }

    pub fn pendadtaran(&self) -> Pendaftaran {
        Pendaftaran::new(Arc::clone(&self.client))
    }

    pub fn spesialis(&self) -> Spesialis {
        Spesialis::new(Arc::clone(&self.client))
    }

    pub async fn request(&self, options: RequestOptions) -> Result<JknResponse> {
        self.client.send(ServiceType::Pcare, options).await
    }
}

#[derive(Clone)]
pub struct Referensi {
    client: Arc<JknClient>,
}

impl Referensi {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn alergi(&self, kode: &str) -> Result<JknResponse> {
        let path = normalize_path("/alergi/jenis/:kode", &[("kode", Some(kode.to_string()))])?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn alergi_typed(&self, kode: &str) -> Result<PcareListResponse<PcareAlergiItem>> {
        self.alergi(kode).await?.into_response()
    }

    pub async fn diagnosa(&self, kode: &str, row: u32, limit: u32) -> Result<JknResponse> {
        let path = normalize_path(
            "/diagnosa/:kode/:row/:limit",
            &[
                ("kode", Some(kode.to_string())),
                ("row", Some(row.to_string())),
                ("limit", Some(limit.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn diagnosa_typed(
        &self,
        kode: &str,
        row: u32,
        limit: u32,
    ) -> Result<PcareListResponse<PcareDiagnosaItem>> {
        self.diagnosa(kode, row, limit).await?.into_response()
    }

    pub async fn dokter(&self, row: u32, limit: u32) -> Result<JknResponse> {
        let path = normalize_path(
            "/dokter/:row/:limit",
            &[
                ("row", Some(row.to_string())),
                ("limit", Some(limit.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn dokter_typed(
        &self,
        row: u32,
        limit: u32,
    ) -> Result<PcareListResponse<PcareDokterItem>> {
        self.dokter(row, limit).await?.into_response()
    }

    pub async fn kesadaran(&self) -> Result<JknResponse> {
        self.client
            .send(ServiceType::Pcare, RequestOptions::get("/kesadaran"))
            .await
    }

    pub async fn kesadaran_typed(&self) -> Result<PcareListResponse<PcareKesadaranItem>> {
        self.kesadaran().await?.into_response()
    }

    pub async fn poli(&self, row: u32, limit: u32) -> Result<JknResponse> {
        let path = normalize_path(
            "/poli/fktp/:row/:limit",
            &[
                ("row", Some(row.to_string())),
                ("limit", Some(limit.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn poli_typed(
        &self,
        row: u32,
        limit: u32,
    ) -> Result<PcareListResponse<PcarePoliItem>> {
        self.poli(row, limit).await?.into_response()
    }

    pub async fn prognosa(&self) -> Result<JknResponse> {
        self.client
            .send(ServiceType::Pcare, RequestOptions::get("/prognosa"))
            .await
    }

    pub async fn prognosa_typed(&self) -> Result<PcareListResponse<PcarePrognosaItem>> {
        self.prognosa().await?.into_response()
    }

    pub async fn status_pulang(&self, ranap: bool) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Pcare,
                RequestOptions::get(format!("/statuspulang/rawatInap/{ranap}")),
            )
            .await
    }

    pub async fn status_pulang_typed(
        &self,
        ranap: bool,
    ) -> Result<PcareListResponse<PcareStatusPulangItem>> {
        self.status_pulang(ranap).await?.into_response()
    }

    pub async fn provider(&self, row: u32, limit: u32) -> Result<JknResponse> {
        let path = normalize_path(
            "/provider/:row/:limit",
            &[
                ("row", Some(row.to_string())),
                ("limit", Some(limit.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn provider_typed(
        &self,
        row: u32,
        limit: u32,
    ) -> Result<PcareListResponse<PcareProviderItem>> {
        self.provider(row, limit).await?.into_response()
    }
}

#[derive(Clone)]
pub struct Peserta {
    client: Arc<JknClient>,
}

impl Peserta {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn nomor_kartu(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path("/peserta/:nomor", &[("nomor", Some(nomor.to_string()))])?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn nomor_kartu_typed(&self, nomor: &str) -> Result<PcarePesertaData> {
        self.nomor_kartu(nomor).await?.into_response()
    }

    pub async fn jenis_kartu(&self, jenis: &str, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/peserta/:jenis/:nomor",
            &[
                ("jenis", Some(jenis.to_string())),
                ("nomor", Some(nomor.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn jenis_kartu_typed(&self, jenis: &str, nomor: &str) -> Result<PcarePesertaData> {
        self.jenis_kartu(jenis, nomor).await?.into_response()
    }
}

#[derive(Clone)]
pub struct Kunjungan {
    client: Arc<JknClient>,
}

impl Kunjungan {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn rujukan(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/kunjungan/rujukan/:nomor",
            &[("nomor", Some(nomor.to_string()))],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn rujukan_typed(&self, nomor: &str) -> Result<PcareRujukanResult> {
        self.rujukan(nomor).await?.into_response()
    }

    pub async fn riwayat(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/kunjungan/peserta/:nomor",
            &[("nomor", Some(nomor.to_string()))],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn riwayat_typed(
        &self,
        nomor: &str,
    ) -> Result<PcareListResponse<PcareRiwayatKunjunganItem>> {
        self.riwayat(nomor).await?.into_response()
    }

    pub async fn insert<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Pcare,
                RequestOptions::post("/kunjungan").data(data)?,
            )
            .await
    }

    pub async fn update<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Pcare,
                RequestOptions::put("/kunjungan").data(data)?,
            )
            .await
    }

    pub async fn delete(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path("/kunjungan/:nomor", &[("nomor", Some(nomor.to_string()))])?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::delete(path))
            .await
    }
}

#[derive(Clone)]
pub struct Spesialis {
    client: Arc<JknClient>,
}

impl Spesialis {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<JknResponse> {
        self.client
            .send(ServiceType::Pcare, RequestOptions::get("/spesialis"))
            .await
    }

    pub async fn get_typed(&self) -> Result<PcareListResponse<PcareSpesialisItem>> {
        self.get().await?.into_response()
    }

    pub async fn sub(&self, kode: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/spesialis/:kode/subspesialis",
            &[("kode", Some(kode.to_string()))],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn sub_typed(&self, kode: &str) -> Result<PcareListResponse<PcareSubSpesialisItem>> {
        self.sub(kode).await?.into_response()
    }

    pub async fn sarana(&self) -> Result<JknResponse> {
        self.client
            .send(ServiceType::Pcare, RequestOptions::get("/spesialis/sarana"))
            .await
    }

    pub async fn sarana_typed(&self) -> Result<PcareListResponse<PcareSaranaItem>> {
        self.sarana().await?.into_response()
    }

    pub async fn khusus(&self) -> Result<JknResponse> {
        self.client
            .send(ServiceType::Pcare, RequestOptions::get("/spesialis/khusus"))
            .await
    }

    pub async fn khusus_typed(&self) -> Result<PcareListResponse<PcareKhususItem>> {
        self.khusus().await?.into_response()
    }

    pub async fn faskes_rujukan_sub_spesialis(
        &self,
        kode_sub: &str,
        kode_sarana: &str,
        tanggal: &str,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/spesialis/rujuk/subspesialis/:kodeSub/sarana/:kodeSarana/tglEstRujuk/:tanggal",
            &[
                ("kodeSub", Some(kode_sub.to_string())),
                ("kodeSarana", Some(kode_sarana.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn faskes_rujukan_sub_spesialis_typed(
        &self,
        kode_sub: &str,
        kode_sarana: &str,
        tanggal: &str,
    ) -> Result<PcareListResponse<PcareFaskesRujukanSubSpesialisItem>> {
        self.faskes_rujukan_sub_spesialis(kode_sub, kode_sarana, tanggal)
            .await?
            .into_response()
    }
}

#[derive(Clone)]
pub struct Pendaftaran {
    client: Arc<JknClient>,
}

impl Pendaftaran {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn get(&self, nomor: &str, tanggal: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/pendaftaran/nourut/:nomor/tgldaftar/:tanggal",
            &[
                ("nomor", Some(nomor.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn get_typed(&self, nomor: &str, tanggal: &str) -> Result<PcarePendaftaranResult> {
        self.get(nomor, tanggal).await?.into_response()
    }

    pub async fn insert<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Pcare,
                RequestOptions::post("/pendaftaran").data(data)?,
            )
            .await
    }

    pub async fn get_provider(&self, tanggal: &str, row: &str, limit: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/pendaftaran/tgldaftar/:tanggal/:row/:limit",
            &[
                ("tanggal", Some(tanggal.to_string())),
                ("row", Some(row.to_string())),
                ("limit", Some(limit.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::get(path))
            .await
    }

    pub async fn get_provider_typed(
        &self,
        tanggal: &str,
        row: &str,
        limit: &str,
    ) -> Result<PcareListResponse<PcarePendaftaranResult>> {
        self.get_provider(tanggal, row, limit)
            .await?
            .into_response()
    }

    pub async fn delete(
        &self,
        noka: &str,
        tanggal: &str,
        nomor: &str,
        poli: &str,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/pendaftaran/peserta/:noka/tglDaftar/:tanggal/noUrut/:nomor/kdPoli/:poli",
            &[
                ("noka", Some(noka.to_string())),
                ("tanggal", Some(tanggal.to_string())),
                ("nomor", Some(nomor.to_string())),
                ("poli", Some(poli.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Pcare, RequestOptions::delete(path))
            .await
    }
}

use std::sync::Arc;

use serde::Serialize;

use crate::JknClient;
use crate::client::{JknResponse, RequestOptions, normalize_path};
use crate::config::ServiceType;
use crate::error::Result;
use crate::models::apotek::{
    ApotekDaftarResepResponse, ApotekDphoResponse, ApotekFaskesResponse, ApotekMonitoringResponse,
    ApotekObatReferensiResponse, ApotekPelayananDaftarResponse, ApotekPoliResponse,
    ApotekPrbRekapPesertaResponse, ApotekResepSimpanResponse, ApotekRiwayatResponse,
    ApotekSepKunjunganResponse, ApotekSettingResponse, ApotekSpesialistikResponse,
};

#[derive(Clone)]
pub struct Apotek {
    client: Arc<JknClient>,
}

impl Apotek {
    pub fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub fn referensi(&self) -> Referensi {
        Referensi::new(Arc::clone(&self.client))
    }

    pub fn obat(&self) -> Obat {
        Obat::new(Arc::clone(&self.client))
    }

    pub fn pelayanan_obat(&self) -> PelayananObat {
        PelayananObat::new(Arc::clone(&self.client))
    }

    pub fn resep(&self) -> Resep {
        Resep::new(Arc::clone(&self.client))
    }

    pub fn sep(&self) -> Sep {
        Sep::new(Arc::clone(&self.client))
    }

    pub fn monitoring(&self) -> Monitoring {
        Monitoring::new(Arc::clone(&self.client))
    }

    pub fn prb(&self) -> Prb {
        Prb::new(Arc::clone(&self.client))
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

    pub async fn dpho(&self) -> Result<JknResponse> {
        self.client
            .send(ServiceType::Apotek, RequestOptions::get("/referensi/dpho"))
            .await
    }

    pub async fn dpho_typed(&self) -> Result<ApotekDphoResponse> {
        self.dpho().await?.into_response()
    }

    pub async fn poli(&self, keyword: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/poli/:keyword",
            &[("keyword", Some(keyword.to_string()))],
        )?;
        self.client
            .send(ServiceType::Apotek, RequestOptions::get(path))
            .await
    }

    pub async fn poli_typed(&self, keyword: &str) -> Result<ApotekPoliResponse> {
        self.poli(keyword).await?.into_response()
    }

    pub async fn faskes(&self, jenis: u32, nama: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/ppk/:jenis/:nama",
            &[
                ("jenis", Some(jenis.to_string())),
                ("nama", Some(nama.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Apotek, RequestOptions::get(path))
            .await
    }

    pub async fn faskes_typed(&self, jenis: u32, nama: &str) -> Result<ApotekFaskesResponse> {
        self.faskes(jenis, nama).await?.into_response()
    }

    pub async fn setting_apotek(&self, kode_apotek: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/settingppk/read/:kodeApotek",
            &[("kodeApotek", Some(kode_apotek.to_string()))],
        )?;
        self.client
            .send(ServiceType::Apotek, RequestOptions::get(path))
            .await
    }

    pub async fn setting_apotek_typed(&self, kode_apotek: &str) -> Result<ApotekSettingResponse> {
        self.setting_apotek(kode_apotek).await?.into_response()
    }

    pub async fn spesialistik(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Apotek,
                RequestOptions::get("/referensi/spesialistik"),
            )
            .await
    }

    pub async fn spesialistik_typed(&self) -> Result<ApotekSpesialistikResponse> {
        self.spesialistik().await?.into_response()
    }

    pub async fn obat(
        &self,
        jenis: &str,
        tanggal: &str,
        filter: Option<&str>,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/obat/:jenis/:tanggal/:filter?",
            &[
                ("jenis", Some(jenis.to_string())),
                ("tanggal", Some(tanggal.to_string())),
                ("filter", filter.map(str::to_string)),
            ],
        )?;
        self.client
            .send(ServiceType::Apotek, RequestOptions::get(path))
            .await
    }

    pub async fn obat_typed(
        &self,
        jenis: &str,
        tanggal: &str,
        filter: Option<&str>,
    ) -> Result<ApotekObatReferensiResponse> {
        self.obat(jenis, tanggal, filter).await?.into_response()
    }
}

#[derive(Clone)]
pub struct Obat {
    client: Arc<JknClient>,
}

impl Obat {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn save_non_racikan<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Apotek,
                RequestOptions::post("/obatnonracikan/v3/insert").data(data)?,
            )
            .await
    }

    pub async fn save_racikan<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Apotek,
                RequestOptions::post("/obatracikan/v3/insert").data(data)?,
            )
            .await
    }

    pub async fn update_stok<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Apotek,
                RequestOptions::post("/UpdateStokObat/updatestok").data(data)?,
            )
            .await
    }
}

#[derive(Clone)]
pub struct Resep {
    client: Arc<JknClient>,
}

impl Resep {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn simpan<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Apotek,
                RequestOptions::post("/sjpresep/v3/insert").data(data)?,
            )
            .await
    }

    pub async fn simpan_typed<T: Serialize>(&self, data: T) -> Result<ApotekResepSimpanResponse> {
        self.simpan(data).await?.into_response()
    }

    pub async fn hapus<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        let request = RequestOptions::delete("/hapusresep")
            .skip_content_type_hack()
            .header("Content-Type", "application/json")?
            .data(data)?;
        self.client.send(ServiceType::Apotek, request).await
    }

    pub async fn daftar<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Apotek,
                RequestOptions::post("/daftarresep").data(data)?,
            )
            .await
    }

    pub async fn daftar_typed<T: Serialize>(&self, data: T) -> Result<ApotekDaftarResepResponse> {
        self.daftar(data).await?.into_response()
    }
}

#[derive(Clone)]
pub struct PelayananObat {
    client: Arc<JknClient>,
}

impl PelayananObat {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn hapus<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        let request = RequestOptions::delete("/pelayanan/obat/hapus/")
            .skip_content_type_hack()
            .header("Content-Type", "application/json")?
            .data(data)?;
        self.client.send(ServiceType::Apotek, request).await
    }

    pub async fn daftar(&self, nomor_sep: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/pelayanan/obat/daftar/:nomorSep",
            &[("nomorSep", Some(nomor_sep.to_string()))],
        )?;
        self.client
            .send(ServiceType::Apotek, RequestOptions::get(path))
            .await
    }

    pub async fn daftar_typed(&self, nomor_sep: &str) -> Result<ApotekPelayananDaftarResponse> {
        self.daftar(nomor_sep).await?.into_response()
    }

    pub async fn riwayat(&self, awal: &str, akhir: &str, nomor_kartu: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/riwayatobat/:awal/:akhir/:nomorKartu",
            &[
                ("awal", Some(awal.to_string())),
                ("akhir", Some(akhir.to_string())),
                ("nomorKartu", Some(nomor_kartu.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Apotek, RequestOptions::get(path))
            .await
    }

    pub async fn riwayat_typed(
        &self,
        awal: &str,
        akhir: &str,
        nomor_kartu: &str,
    ) -> Result<ApotekRiwayatResponse> {
        self.riwayat(awal, akhir, nomor_kartu)
            .await?
            .into_response()
    }
}

#[derive(Clone)]
pub struct Sep {
    client: Arc<JknClient>,
}

impl Sep {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn kunjungan(&self, nomor_sep: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/sep/:nomorSep",
            &[("nomorSep", Some(nomor_sep.to_string()))],
        )?;
        self.client
            .send(ServiceType::Apotek, RequestOptions::get(path))
            .await
    }

    pub async fn kunjungan_typed(&self, nomor_sep: &str) -> Result<ApotekSepKunjunganResponse> {
        self.kunjungan(nomor_sep).await?.into_response()
    }
}

#[derive(Clone)]
pub struct Monitoring {
    client: Arc<JknClient>,
}

impl Monitoring {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn data_klaim(
        &self,
        bulan: u32,
        tahun: u32,
        jenis_obat: u32,
        status: u32,
    ) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Apotek,
                RequestOptions::get(format!(
                    "/monitoring/klaim/{bulan}/{tahun}/{jenis_obat}/{status}"
                )),
            )
            .await
    }

    pub async fn data_klaim_typed(
        &self,
        bulan: u32,
        tahun: u32,
        jenis_obat: u32,
        status: u32,
    ) -> Result<ApotekMonitoringResponse> {
        self.data_klaim(bulan, tahun, jenis_obat, status)
            .await?
            .into_response()
    }
}

#[derive(Clone)]
pub struct Prb {
    client: Arc<JknClient>,
}

impl Prb {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn rekap_peserta(&self, tahun: u32, bulan: u32) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Apotek,
                RequestOptions::get(format!("/Prb/rekappeserta/tahun/{tahun}/bulan/{bulan}")),
            )
            .await
    }

    pub async fn rekap_peserta_typed(
        &self,
        tahun: u32,
        bulan: u32,
    ) -> Result<ApotekPrbRekapPesertaResponse> {
        self.rekap_peserta(tahun, bulan).await?.into_response()
    }
}

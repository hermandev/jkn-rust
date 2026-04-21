use std::sync::Arc;

use serde_json::json;

use crate::JknClient;
use crate::client::{JknResponse, RequestOptions, normalize_path};
use crate::config::ServiceType;
use crate::error::Result;
use crate::models::antrean::{
    AntreanBatalRequest, AntreanDashboardList, AntreanDetail, AntreanRefDokterItem,
    AntreanRefJadwalDokterItem, AntreanRefPasienFp, AntreanRefPoliFpItem, AntreanRefPoliItem,
    AntreanTambahFarmasiRequest, AntreanTambahRequest, AntreanTaskItem, AntreanUpdateWaktuRequest,
};

#[derive(Clone)]
pub struct Antrean {
    client: Arc<JknClient>,
}

impl Antrean {
    pub fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn ref_poli(&self) -> Result<JknResponse> {
        self.client
            .send(ServiceType::Antrean, RequestOptions::get("/ref/poli"))
            .await
    }

    pub async fn ref_poli_typed(&self) -> Result<Vec<AntreanRefPoliItem>> {
        self.ref_poli().await?.into_response()
    }

    pub async fn ref_dokter(&self) -> Result<JknResponse> {
        self.client
            .send(ServiceType::Antrean, RequestOptions::get("/ref/dokter"))
            .await
    }

    pub async fn ref_dokter_typed(&self) -> Result<Vec<AntreanRefDokterItem>> {
        self.ref_dokter().await?.into_response()
    }

    pub async fn ref_jadwal_dokter(&self, poli: &str, tanggal: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/jadwaldokter/kodepoli/:poli/tanggal/:tanggal",
            &[
                ("poli", Some(poli.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Antrean, RequestOptions::get(path))
            .await
    }

    pub async fn ref_jadwal_dokter_typed(
        &self,
        poli: &str,
        tanggal: &str,
    ) -> Result<Vec<AntreanRefJadwalDokterItem>> {
        self.ref_jadwal_dokter(poli, tanggal).await?.into_response()
    }

    pub async fn ref_poli_fp(&self) -> Result<JknResponse> {
        self.client
            .send(ServiceType::Antrean, RequestOptions::get("/ref/poli/fp"))
            .await
    }

    pub async fn ref_poli_fp_typed(&self) -> Result<Vec<AntreanRefPoliFpItem>> {
        self.ref_poli_fp().await?.into_response()
    }

    pub async fn ref_pasien_fp(&self, jenis: &str, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/ref/pasien/fp/identitas/:jenis/noidentitas/:nomor",
            &[
                ("jenis", Some(jenis.to_string())),
                ("nomor", Some(nomor.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Antrean, RequestOptions::get(path))
            .await
    }

    pub async fn ref_pasien_fp_typed(
        &self,
        jenis: &str,
        nomor: &str,
    ) -> Result<AntreanRefPasienFp> {
        self.ref_pasien_fp(jenis, nomor).await?.into_response()
    }

    pub async fn update_jadwal_dokter<T: serde::Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::post("/jadwaldokter/updatejadwaldokter").data(data)?,
            )
            .await
    }

    pub async fn tambah<T: serde::Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::post("/antrean/add").data(data)?,
            )
            .await
    }

    pub async fn tambah_typed(&self, data: AntreanTambahRequest) -> Result<JknResponse> {
        self.tambah(data).await
    }

    pub async fn tambah_farmasi<T: serde::Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::post("/antrean/farmasi/add").data(data)?,
            )
            .await
    }

    pub async fn tambah_farmasi_typed(
        &self,
        data: AntreanTambahFarmasiRequest,
    ) -> Result<JknResponse> {
        self.tambah_farmasi(data).await
    }

    pub async fn update_waktu<T: serde::Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::post("/antrean/updatewaktu").data(data)?,
            )
            .await
    }

    pub async fn update_waktu_typed(&self, data: AntreanUpdateWaktuRequest) -> Result<JknResponse> {
        self.update_waktu(data).await
    }

    pub async fn batal<T: serde::Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::post("/antrean/batal").data(data)?,
            )
            .await
    }

    pub async fn batal_typed(&self, data: AntreanBatalRequest) -> Result<JknResponse> {
        self.batal(data).await
    }

    pub async fn list_task_id(&self, kode_booking: &str) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::post("/antrean/getlisttask")
                    .data(json!({ "kodebooking": kode_booking }))?,
            )
            .await
    }

    pub async fn list_task_id_typed(&self, kode_booking: &str) -> Result<Vec<AntreanTaskItem>> {
        self.list_task_id(kode_booking).await?.into_response()
    }

    pub async fn dashboard_per_tanggal(&self, tanggal: &str, waktu: &str) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::get(format!(
                    "/dashboard/waktutunggu/tanggal/{tanggal}/waktu/{waktu}"
                ))
                .skip_decrypt(),
            )
            .await
    }

    pub async fn dashboard_per_tanggal_typed(
        &self,
        tanggal: &str,
        waktu: &str,
    ) -> Result<AntreanDashboardList> {
        self.dashboard_per_tanggal(tanggal, waktu)
            .await?
            .into_response()
    }

    pub async fn dashboard_per_bulan(
        &self,
        bulan: u32,
        tahun: u32,
        waktu: &str,
    ) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::get(format!(
                    "/dashboard/waktutunggu/bulan/{:02}/tahun/{tahun}/waktu/{waktu}",
                    bulan
                ))
                .skip_decrypt(),
            )
            .await
    }

    pub async fn dashboard_per_bulan_typed(
        &self,
        bulan: u32,
        tahun: u32,
        waktu: &str,
    ) -> Result<AntreanDashboardList> {
        self.dashboard_per_bulan(bulan, tahun, waktu)
            .await?
            .into_response()
    }

    pub async fn per_tanggal(&self, tanggal: &str) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::get(format!("/antrean/pendaftaran/tanggal/{tanggal}")),
            )
            .await
    }

    pub async fn per_tanggal_typed(&self, tanggal: &str) -> Result<Vec<AntreanDetail>> {
        self.per_tanggal(tanggal).await?.into_response()
    }

    pub async fn per_kode_booking(&self, kode_booking: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/antrean/pendaftaran/kodebooking/:kodeBooking",
            &[("kodeBooking", Some(kode_booking.to_string()))],
        )?;
        self.client
            .send(ServiceType::Antrean, RequestOptions::get(path))
            .await
    }

    pub async fn per_kode_booking_typed(&self, kode_booking: &str) -> Result<Vec<AntreanDetail>> {
        self.per_kode_booking(kode_booking).await?.into_response()
    }

    pub async fn belum_dilayani(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Antrean,
                RequestOptions::get("/antrean/pendaftaran/aktif"),
            )
            .await
    }

    pub async fn belum_dilayani_typed(&self) -> Result<Vec<AntreanDetail>> {
        self.belum_dilayani().await?.into_response()
    }

    pub async fn belum_dilayani_predikat(
        &self,
        poli: &str,
        dokter: &str,
        hari: u32,
        jam: &str,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/antrean/pendaftaran/kodepoli/:poli/kodedokter/:dokter/hari/:hari/jampraktek/:jam",
            &[
                ("poli", Some(poli.to_string())),
                ("dokter", Some(dokter.to_string())),
                ("hari", Some(hari.to_string())),
                ("jam", Some(jam.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Antrean, RequestOptions::get(path))
            .await
    }

    pub async fn belum_dilayani_predikat_typed(
        &self,
        poli: &str,
        dokter: &str,
        hari: u32,
        jam: &str,
    ) -> Result<Vec<AntreanDetail>> {
        self.belum_dilayani_predikat(poli, dokter, hari, jam)
            .await?
            .into_response()
    }
}

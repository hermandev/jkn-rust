use std::sync::Arc;

use serde::Serialize;
use serde_json::json;

use crate::JknClient;
use crate::client::{JknResponse, RequestOptions, normalize_path};
use crate::config::ServiceType;
use crate::error::{JknError, Result};
use crate::models::vclaim::{
    ReferensiDiagnosaResponse, ReferensiDpjpResponse, ReferensiFaskesResponse, ReferensiList,
    ReferensiPoliResponse, VclaimFingerPrintPesertaListResponse, VclaimFingerPrintStatusResponse,
    VclaimInacbgResponse, VclaimIndukKecelakaanResponse, VclaimInternalSepListResponse,
    VclaimJumlahSepResponse, VclaimPersetujuanSepListResponse, VclaimPesertaResponse,
    VclaimRandomQuestionResponse, VclaimRujukanKeluarDetail, VclaimRujukanKeluarListResponse,
    VclaimRujukanKhususListResponse, VclaimRujukanListResponse, VclaimRujukanResponse,
    VclaimRujukanSaranaListResponse, VclaimRujukanSpesialistikListResponse, VclaimSepDetail,
    VclaimSepWriteSimpleResponse, VclaimSuplesiJasaRaharjaResponse,
    VclaimTanggalPulangListResponse,
};

#[derive(Clone)]
pub struct VClaim {
    client: Arc<JknClient>,
}

impl VClaim {
    pub fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub fn lpk(&self) -> Lpk {
        Lpk::new(Arc::clone(&self.client))
    }

    pub fn monitoring(&self) -> Monitoring {
        Monitoring::new(Arc::clone(&self.client))
    }

    pub fn peserta(&self) -> Peserta {
        Peserta::new(Arc::clone(&self.client))
    }

    pub fn prb(&self) -> Prb {
        Prb::new(Arc::clone(&self.client))
    }

    pub fn referensi(&self) -> Referensi {
        Referensi::new(Arc::clone(&self.client))
    }

    pub fn rencana_kontrol(&self) -> RencanaKontrol {
        RencanaKontrol::new(Arc::clone(&self.client))
    }

    pub fn rujukan(&self) -> Rujukan {
        Rujukan::new(Arc::clone(&self.client))
    }

    pub fn sep(&self) -> Sep {
        Sep::new(Arc::clone(&self.client))
    }
}

fn wrap_t_lpk<T: Serialize>(data: T) -> serde_json::Value {
    json!({ "request": { "t_lpk": data } })
}

fn wrap_t_prb<T: Serialize>(data: T) -> serde_json::Value {
    json!({ "request": { "t_prb": data } })
}

fn wrap_t_rujukan<T: Serialize>(data: T) -> serde_json::Value {
    json!({ "request": { "t_rujukan": data } })
}

fn wrap_t_sep<T: Serialize>(data: T) -> serde_json::Value {
    json!({ "request": { "t_sep": data } })
}

#[derive(Clone)]
pub struct Referensi {
    client: Arc<JknClient>,
}

impl Referensi {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn diagnosa(&self, keyword: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/diagnosa/:keyword",
            &[("keyword", Some(keyword.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn diagnosa_typed(&self, keyword: &str) -> Result<ReferensiDiagnosaResponse> {
        self.diagnosa(keyword).await?.into_response()
    }

    pub async fn poli(&self, keyword: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/poli/:keyword",
            &[("keyword", Some(keyword.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn poli_typed(&self, keyword: &str) -> Result<ReferensiPoliResponse> {
        self.poli(keyword).await?.into_response()
    }

    pub async fn faskes(&self, keyword: &str, jenis: u32) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/faskes/:keyword/:jenis",
            &[
                ("keyword", Some(keyword.to_string())),
                ("jenis", Some(jenis.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn faskes_typed(&self, keyword: &str, jenis: u32) -> Result<ReferensiFaskesResponse> {
        self.faskes(keyword, jenis).await?.into_response()
    }

    pub async fn dpjp(&self, jenis: u32, tanggal: &str, kode: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/dokter/pelayanan/:jenis/tglPelayanan/:tanggal/Spesialis/:kode",
            &[
                ("jenis", Some(jenis.to_string())),
                ("tanggal", Some(tanggal.to_string())),
                ("kode", Some(kode.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn dpjp_typed(
        &self,
        jenis: u32,
        tanggal: &str,
        kode: &str,
    ) -> Result<ReferensiDpjpResponse> {
        self.dpjp(jenis, tanggal, kode).await?.into_response()
    }

    pub async fn provinsi(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get("/referensi/propinsi"),
            )
            .await
    }

    pub async fn provinsi_typed(&self) -> Result<ReferensiList> {
        self.provinsi().await?.into_response()
    }

    pub async fn kabupaten(&self, provinsi: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/kabupaten/propinsi/:provinsi",
            &[("provinsi", Some(provinsi.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn kabupaten_typed(&self, provinsi: &str) -> Result<ReferensiList> {
        self.kabupaten(provinsi).await?.into_response()
    }

    pub async fn kecamatan(&self, kabupaten: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/kecamatan/kabupaten/:kabupaten",
            &[("kabupaten", Some(kabupaten.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn kecamatan_typed(&self, kabupaten: &str) -> Result<ReferensiList> {
        self.kecamatan(kabupaten).await?.into_response()
    }

    pub async fn diagnosa_prb(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get("/referensi/diagnosaprb"),
            )
            .await
    }

    pub async fn diagnosa_prb_typed(&self) -> Result<ReferensiList> {
        self.diagnosa_prb().await?.into_response()
    }

    pub async fn obat_prb(&self, nama: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/obatprb/:nama",
            &[("nama", Some(nama.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn obat_prb_typed(&self, nama: &str) -> Result<ReferensiList> {
        self.obat_prb(nama).await?.into_response()
    }

    pub async fn klaim_prosedur(&self, keyword: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/procedure/:keyword",
            &[("keyword", Some(keyword.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn klaim_prosedur_typed(&self, keyword: &str) -> Result<ReferensiList> {
        self.klaim_prosedur(keyword).await?.into_response()
    }

    pub async fn klaim_kelas_rawat(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get("/referensi/kelasrawat"),
            )
            .await
    }

    pub async fn klaim_kelas_rawat_typed(&self) -> Result<ReferensiList> {
        self.klaim_kelas_rawat().await?.into_response()
    }

    pub async fn klaim_dokter(&self, nama: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/referensi/dokter/:nama",
            &[("nama", Some(nama.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn klaim_dokter_typed(&self, nama: &str) -> Result<ReferensiList> {
        self.klaim_dokter(nama).await?.into_response()
    }

    pub async fn klaim_spesialistik(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get("/referensi/spesialistik"),
            )
            .await
    }

    pub async fn klaim_spesialistik_typed(&self) -> Result<ReferensiList> {
        self.klaim_spesialistik().await?.into_response()
    }

    pub async fn klaim_ruang_rawat(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get("/referensi/ruangrawat"),
            )
            .await
    }

    pub async fn klaim_ruang_rawat_typed(&self) -> Result<ReferensiList> {
        self.klaim_ruang_rawat().await?.into_response()
    }

    pub async fn klaim_cara_keluar(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get("/referensi/carakeluar"),
            )
            .await
    }

    pub async fn klaim_cara_keluar_typed(&self) -> Result<ReferensiList> {
        self.klaim_cara_keluar().await?.into_response()
    }

    pub async fn klaim_paska_pulang(&self) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get("/referensi/pascapulang"),
            )
            .await
    }

    pub async fn klaim_paska_pulang_typed(&self) -> Result<ReferensiList> {
        self.klaim_paska_pulang().await?.into_response()
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

    pub async fn nomor_kartu(&self, nomor: &str, tanggal: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/Peserta/nokartu/:nomor/tglSEP/:tanggal",
            &[
                ("nomor", Some(nomor.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn nomor_kartu_typed(
        &self,
        nomor: &str,
        tanggal: &str,
    ) -> Result<VclaimPesertaResponse> {
        self.nomor_kartu(nomor, tanggal).await?.into_response()
    }

    pub async fn nomor_kependudukan(&self, nomor: &str, tanggal: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/Peserta/nik/:nomor/tglSEP/:tanggal",
            &[
                ("nomor", Some(nomor.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn nomor_kependudukan_typed(
        &self,
        nomor: &str,
        tanggal: &str,
    ) -> Result<VclaimPesertaResponse> {
        self.nomor_kependudukan(nomor, tanggal)
            .await?
            .into_response()
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

    pub async fn kunjungan(&self, tanggal: &str, jenis: u32) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get(format!(
                    "/Monitoring/Kunjungan/Tanggal/{tanggal}/JnsPelayanan/{jenis}"
                )),
            )
            .await
    }

    pub async fn klaim(&self, tanggal: &str, jenis: u32, status: u32) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get(format!(
                    "/Monitoring/Klaim/Tanggal/{tanggal}/JnsPelayanan/{jenis}/Status/{status}"
                )),
            )
            .await
    }

    pub async fn riwayat_pelayanan(
        &self,
        nomor_kartu: &str,
        awal: &str,
        akhir: &str,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/monitoring/HistoriPelayanan/NoKartu/:nomorKartu/tglMulai/:awal/tglAkhir/:akhir",
            &[
                ("nomorKartu", Some(nomor_kartu.to_string())),
                ("awal", Some(awal.to_string())),
                ("akhir", Some(akhir.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn klaim_jasa_raharja(
        &self,
        jenis: u32,
        awal: &str,
        akhir: &str,
    ) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get(format!(
                    "/monitoring/JasaRaharja/JnsPelayanan/{jenis}/tglMulai/{awal}/tglAkhir/{akhir}"
                )),
            )
            .await
    }
}

#[derive(Clone)]
pub struct Lpk {
    client: Arc<JknClient>,
}

impl Lpk {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub async fn insert<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/LPK/insert").data(wrap_t_lpk(data))?,
            )
            .await
    }

    pub async fn update<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/LPK/update").data(wrap_t_lpk(data))?,
            )
            .await
    }

    pub async fn delete(&self, nomor_sep: &str) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::delete("/LPK/delete")
                    .data(json!({ "request": { "t_lpk": { "noSep": nomor_sep }}}))?,
            )
            .await
    }

    pub async fn data(&self, tanggal: &str, jenis: u32) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get(format!("/LPK/TglMasuk/{tanggal}/JnsPelayanan/{jenis}")),
            )
            .await
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

    pub async fn insert<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/PRB/insert").data(wrap_t_prb(data))?,
            )
            .await
    }

    pub async fn update<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/PRB/Update").data(wrap_t_prb(data))?,
            )
            .await
    }

    pub async fn delete<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::delete("/PRB/Delete").data(wrap_t_prb(data))?,
            )
            .await
    }

    pub async fn cari_by_nomor(&self, nomor_srb: &str, nomor_sep: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/prb/:nomorSrb/nosep/:nomorSep",
            &[
                ("nomorSrb", Some(nomor_srb.to_string())),
                ("nomorSep", Some(nomor_sep.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn cari_by_tanggal(&self, awal: &str, akhir: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/prb/tglMulai/:awal/tglAkhir/:akhir",
            &[
                ("awal", Some(awal.to_string())),
                ("akhir", Some(akhir.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn rekap_potensi(&self, tahun: u32, bulan: u32) -> Result<JknResponse> {
        let path = normalize_path(
            "/prbpotensi/tahun/:tahun/bulan/:bulan",
            &[
                ("tahun", Some(tahun.to_string())),
                ("bulan", Some(bulan.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }
}

#[derive(Clone)]
pub struct RencanaKontrol {
    client: Arc<JknClient>,
}

impl RencanaKontrol {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    pub fn list_penyakit_prb(&self) -> Vec<(&'static str, &'static str)> {
        vec![
            ("01", "Diabetes Mellitus"),
            ("02", "Hipertensi"),
            ("03", "Asma"),
            ("04", "Penyakit Jantung"),
            ("05", "PPOK"),
            ("06", "Skizofrenia"),
            ("07", "Stroke"),
            ("08", "Epilepsi"),
            ("09", "SLE"),
        ]
    }

    pub fn all_fields_penyakit_prb(
        &self,
    ) -> Vec<(&'static str, &'static [&'static str], f32, f32)> {
        vec![
            ("HBA1C", &["01"], 0.1, 15.0),
            ("GDP", &["01", "07"], 10.0, 500.0),
            ("GD2JPP", &["01"], 10.0, 500.0),
            ("eGFR", &["01", "02"], 5.0, 150.0),
            ("TD_Sistolik", &["01", "07"], 20.0, 200.0),
            ("TD_Diastolik", &["01", "07"], 20.0, 200.0),
        ]
    }

    pub async fn insert<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/RencanaKontrol/insert").data(json!({ "request": data }))?,
            )
            .await
    }

    pub async fn insert_v2<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/RencanaKontrol/v2/Insert")
                    .data(json!({ "request": data }))?,
            )
            .await
    }

    pub async fn update<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/RencanaKontrol/Update").data(json!({ "request": data }))?,
            )
            .await
    }

    pub async fn update_v2<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/RencanaKontrol/v2/Update")
                    .data(json!({ "request": data }))?,
            )
            .await
    }

    pub async fn delete(&self, no_surat_kontrol: &str, user: &str) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::delete("/RencanaKontrol/Delete")
                    .data(json!({ "request": { "t_suratkontrol": { "noSuratKontrol": no_surat_kontrol, "user": user }}}))?,
            )
            .await
    }

    pub async fn insert_spri<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/RencanaKontrol/InsertSPRI")
                    .data(json!({ "request": data }))?,
            )
            .await
    }

    pub async fn update_spri<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/RencanaKontrol/UpdateSPRI")
                    .data(json!({ "request": data }))?,
            )
            .await
    }

    pub async fn sep(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/RencanaKontrol/nosep/:nomor",
            &[("nomor", Some(nomor.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn cari(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/RencanaKontrol/noSuratKontrol/:nomor",
            &[("nomor", Some(nomor.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn data_by_noka(
        &self,
        bulan: u32,
        tahun: u32,
        nomor_kartu: &str,
        filter: u32,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/RencanaKontrol/ListRencanaKontrol/Bulan/:bulan/Tahun/:tahun/Nokartu/:nomorKartu/filter/:filter",
            &[
                ("bulan", Some(format!("{bulan:02}"))),
                ("tahun", Some(tahun.to_string())),
                ("nomorKartu", Some(nomor_kartu.to_string())),
                ("filter", Some(filter.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn data_by_tanggal(
        &self,
        awal: &str,
        akhir: &str,
        filter: u32,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/RencanaKontrol/ListRencanaKontrol/tglAwal/:awal/tglAkhir/:akhir/filter/:filter",
            &[
                ("awal", Some(awal.to_string())),
                ("akhir", Some(akhir.to_string())),
                ("filter", Some(filter.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn poli(&self, jenis: u32, nomor: &str, tanggal: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/RencanaKontrol/ListSpesialistik/JnsKontrol/:jenis/nomor/:nomor/TglRencanaKontrol/:tanggal",
            &[
                ("jenis", Some(jenis.to_string())),
                ("nomor", Some(nomor.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn dokter(&self, jenis: u32, kode_poli: &str, tanggal: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/RencanaKontrol/JadwalPraktekDokter/JnsKontrol/:jenis/KdPoli/:kodePoli/TglRencanaKontrol/:tanggal",
            &[
                ("jenis", Some(jenis.to_string())),
                ("kodePoli", Some(kode_poli.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }
}

#[derive(Clone)]
pub struct Rujukan {
    client: Arc<JknClient>,
}

impl Rujukan {
    fn new(client: Arc<JknClient>) -> Self {
        Self { client }
    }

    fn validate_sumber(&self, sumber: u32) -> Result<()> {
        if matches!(sumber, 1 | 2) {
            Ok(())
        } else {
            Err(JknError::InvalidArgument(
                "nilai sumber faskes harus 1 atau 2".to_string(),
            ))
        }
    }

    pub async fn cari_by_nomor(&self, nomor: &str, sumber: u32) -> Result<JknResponse> {
        self.validate_sumber(sumber)?;
        let path = if sumber == 1 {
            "/Rujukan/:nomor"
        } else {
            "/Rujukan/RS/:nomor"
        };
        let path = normalize_path(path, &[("nomor", Some(nomor.to_string()))])?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn cari_by_nomor_typed(
        &self,
        nomor: &str,
        sumber: u32,
    ) -> Result<VclaimRujukanResponse> {
        self.cari_by_nomor(nomor, sumber).await?.into_response()
    }

    pub async fn cari_by_noka(&self, nomor: &str, sumber: u32) -> Result<JknResponse> {
        self.validate_sumber(sumber)?;
        let path = if sumber == 1 {
            "/Rujukan/Peserta/:nomor"
        } else {
            "/Rujukan/RS/Peserta/:nomor"
        };
        let path = normalize_path(path, &[("nomor", Some(nomor.to_string()))])?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn cari_by_noka_typed(
        &self,
        nomor: &str,
        sumber: u32,
    ) -> Result<VclaimRujukanResponse> {
        self.cari_by_noka(nomor, sumber).await?.into_response()
    }

    pub async fn cari_by_noka_multi(&self, nomor: &str, sumber: u32) -> Result<JknResponse> {
        self.validate_sumber(sumber)?;
        let path = if sumber == 1 {
            "/Rujukan/List/Peserta/:nomor"
        } else {
            "/Rujukan/RS/List/Peserta/:nomor"
        };
        let path = normalize_path(path, &[("nomor", Some(nomor.to_string()))])?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn cari_by_noka_multi_typed(
        &self,
        nomor: &str,
        sumber: u32,
    ) -> Result<VclaimRujukanListResponse> {
        self.cari_by_noka_multi(nomor, sumber)
            .await?
            .into_response()
    }

    pub async fn insert<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/Rujukan/insert").data(wrap_t_rujukan(data))?,
            )
            .await
    }

    pub async fn insert_typed<T: Serialize>(
        &self,
        data: T,
    ) -> Result<VclaimSepWriteSimpleResponse> {
        self.insert(data).await?.into_response()
    }

    pub async fn update<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/Rujukan/update").data(wrap_t_rujukan(data))?,
            )
            .await
    }

    pub async fn delete<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::delete("/Rujukan/delete").data(wrap_t_rujukan(data))?,
            )
            .await
    }

    pub async fn insert_khusus<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/Rujukan/Khusus/insert").data(data)?,
            )
            .await
    }

    pub async fn delete_khusus(
        &self,
        id_rujukan: &str,
        no_rujukan: &str,
        user: &str,
    ) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/Rujukan/Khusus/delete")
                    .data(json!({ "request": { "t_rujukan": { "idRujukan": id_rujukan, "noRujukan": no_rujukan, "user": user }}}))?,
            )
            .await
    }

    pub async fn list_khusus(&self, bulan: u32, tahun: u32) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get(format!("/Rujukan/Khusus/List/Bulan/{bulan}/Tahun/{tahun}")),
            )
            .await
    }

    pub async fn list_khusus_typed(
        &self,
        bulan: u32,
        tahun: u32,
    ) -> Result<VclaimRujukanKhususListResponse> {
        self.list_khusus(bulan, tahun).await?.into_response()
    }

    pub async fn insert_v2<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/Rujukan/2.0/insert").data(wrap_t_rujukan(data))?,
            )
            .await
    }

    pub async fn insert_v2_typed<T: Serialize>(
        &self,
        data: T,
    ) -> Result<VclaimSepWriteSimpleResponse> {
        self.insert_v2(data).await?.into_response()
    }

    pub async fn update_v2<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/Rujukan/2.0/Update").data(wrap_t_rujukan(data))?,
            )
            .await
    }

    pub async fn list_spesialistik(&self, kode_ppk: &str, tanggal: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/Rujukan/ListSpesialistik/PPKRujukan/:kodePpk/TglRujukan/:tanggal",
            &[
                ("kodePpk", Some(kode_ppk.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn list_spesialistik_typed(
        &self,
        kode_ppk: &str,
        tanggal: &str,
    ) -> Result<VclaimRujukanSpesialistikListResponse> {
        self.list_spesialistik(kode_ppk, tanggal)
            .await?
            .into_response()
    }

    pub async fn list_sarana(&self, kode_ppk: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/Rujukan/ListSarana/PPKRujukan/:kodePpk",
            &[("kodePpk", Some(kode_ppk.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn list_sarana_typed(
        &self,
        kode_ppk: &str,
    ) -> Result<VclaimRujukanSaranaListResponse> {
        self.list_sarana(kode_ppk).await?.into_response()
    }

    pub async fn list_keluar(&self, awal: &str, akhir: &str) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get(format!(
                    "/Rujukan/Keluar/List/tglMulai/{awal}/tglAkhir/{akhir}"
                )),
            )
            .await
    }

    pub async fn list_keluar_typed(
        &self,
        awal: &str,
        akhir: &str,
    ) -> Result<VclaimRujukanKeluarListResponse> {
        self.list_keluar(awal, akhir).await?.into_response()
    }

    pub async fn keluar_by_nomor(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/Rujukan/Keluar/:nomor",
            &[("nomor", Some(nomor.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn keluar_by_nomor_typed(&self, nomor: &str) -> Result<VclaimRujukanKeluarDetail> {
        self.keluar_by_nomor(nomor).await?.into_response()
    }

    pub async fn jumlah_sep(&self, jenis: u32, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/Rujukan/JumlahSEP/:jenis/:nomor",
            &[
                ("jenis", Some(jenis.to_string())),
                ("nomor", Some(nomor.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn jumlah_sep_typed(
        &self,
        jenis: u32,
        nomor: &str,
    ) -> Result<VclaimJumlahSepResponse> {
        self.jumlah_sep(jenis, nomor).await?.into_response()
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

    pub async fn insert<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/SEP/1.1/insert").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn insert_typed<T: Serialize>(
        &self,
        data: T,
    ) -> Result<VclaimSepWriteSimpleResponse> {
        self.insert(data).await?.into_response()
    }

    pub async fn update<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/SEP/1.1/Update").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn delete<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::delete("/SEP/Delete").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn cari(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path("/SEP/:nomor", &[("nomor", Some(nomor.to_string()))])?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn cari_typed(&self, nomor: &str) -> Result<VclaimSepDetail> {
        self.cari(nomor).await?.into_response()
    }

    pub async fn cari_by_rujukan(&self, nomor_rujukan: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/Rujukan/lastsep/norujukan/:nomorRujukan",
            &[("nomorRujukan", Some(nomor_rujukan.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn cari_by_rujukan_typed(&self, nomor_rujukan: &str) -> Result<VclaimSepDetail> {
        self.cari_by_rujukan(nomor_rujukan).await?.into_response()
    }

    pub async fn insert_v2<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/SEP/2.0/insert").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn insert_v2_typed<T: Serialize>(
        &self,
        data: T,
    ) -> Result<VclaimSepWriteSimpleResponse> {
        self.insert_v2(data).await?.into_response()
    }

    pub async fn update_v2<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/SEP/2.0/update").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn delete_v2<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::delete("/SEP/2.0/delete")
                    .skip_decrypt()
                    .data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn suplesi_jasa_raharja(
        &self,
        nomor_kartu: &str,
        tanggal_pelayanan: &str,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/sep/JasaRaharja/Suplesi/:nomorKartu/tglPelayanan/:tanggalPelayanan",
            &[
                ("nomorKartu", Some(nomor_kartu.to_string())),
                ("tanggalPelayanan", Some(tanggal_pelayanan.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn suplesi_jasa_raharja_typed(
        &self,
        nomor_kartu: &str,
        tanggal_pelayanan: &str,
    ) -> Result<VclaimSuplesiJasaRaharjaResponse> {
        self.suplesi_jasa_raharja(nomor_kartu, tanggal_pelayanan)
            .await?
            .into_response()
    }

    pub async fn data_induk_kecelakaan(&self, nomor_kartu: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/sep/KllInduk/List/:nomorKartu",
            &[("nomorKartu", Some(nomor_kartu.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn data_induk_kecelakaan_typed(
        &self,
        nomor_kartu: &str,
    ) -> Result<VclaimIndukKecelakaanResponse> {
        self.data_induk_kecelakaan(nomor_kartu)
            .await?
            .into_response()
    }

    pub async fn pengajuan<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/Sep/pengajuanSEP").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn pengajuan_typed<T: Serialize>(
        &self,
        data: T,
    ) -> Result<VclaimSepWriteSimpleResponse> {
        self.pengajuan(data).await?.into_response()
    }

    pub async fn approval_pengajuan<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/Sep/aprovalSEP").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn approval_pengajuan_typed<T: Serialize>(
        &self,
        data: T,
    ) -> Result<VclaimSepWriteSimpleResponse> {
        self.approval_pengajuan(data).await?.into_response()
    }

    pub async fn list_persetujuan(&self, bulan: u32, tahun: u32) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get(format!(
                    "/Sep/persetujuanSEP/list/bulan/{:02}/tahun/{tahun}",
                    bulan
                )),
            )
            .await
    }

    pub async fn list_persetujuan_typed(
        &self,
        bulan: u32,
        tahun: u32,
    ) -> Result<VclaimPersetujuanSepListResponse> {
        self.list_persetujuan(bulan, tahun).await?.into_response()
    }

    pub async fn update_tanggal_pulang<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/Sep/updtglplg").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn update_tanggal_pulang_v2<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::put("/SEP/2.0/updtglplg").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn list_tanggal_pulang(
        &self,
        bulan: u32,
        tahun: u32,
        filter: Option<&str>,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/Sep/updtglplg/list/bulan/:bulan/tahun/:tahun/:filter?",
            &[
                ("bulan", Some(format!("{bulan:02}"))),
                ("tahun", Some(tahun.to_string())),
                ("filter", filter.map(str::to_string)),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn list_tanggal_pulang_typed(
        &self,
        bulan: u32,
        tahun: u32,
        filter: Option<&str>,
    ) -> Result<VclaimTanggalPulangListResponse> {
        self.list_tanggal_pulang(bulan, tahun, filter)
            .await?
            .into_response()
    }

    pub async fn inacbg(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path("/sep/cbg/:nomor", &[("nomor", Some(nomor.to_string()))])?;
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::get(path).skip_decrypt(),
            )
            .await
    }

    pub async fn inacbg_typed(&self, nomor: &str) -> Result<VclaimInacbgResponse> {
        self.inacbg(nomor).await?.into_response()
    }

    pub async fn list_internal(&self, nomor: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/SEP/Internal/:nomor",
            &[("nomor", Some(nomor.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn list_internal_typed(&self, nomor: &str) -> Result<VclaimInternalSepListResponse> {
        self.list_internal(nomor).await?.into_response()
    }

    pub async fn delete_internal<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::delete("/SEP/Internal/delete").data(wrap_t_sep(data))?,
            )
            .await
    }

    pub async fn finger_print(&self, nomor_kartu: &str, tanggal: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/SEP/FingerPrint/Peserta/:nomorKartu/TglPelayanan/:tanggal",
            &[
                ("nomorKartu", Some(nomor_kartu.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn finger_print_typed(
        &self,
        nomor_kartu: &str,
        tanggal: &str,
    ) -> Result<VclaimFingerPrintStatusResponse> {
        self.finger_print(nomor_kartu, tanggal)
            .await?
            .into_response()
    }

    pub async fn list_finger_print(&self, tanggal: &str) -> Result<JknResponse> {
        let path = normalize_path(
            "/SEP/FingerPrint/List/Peserta/TglPelayanan/:tanggal",
            &[("tanggal", Some(tanggal.to_string()))],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn list_finger_print_typed(
        &self,
        tanggal: &str,
    ) -> Result<VclaimFingerPrintPesertaListResponse> {
        self.list_finger_print(tanggal).await?.into_response()
    }

    pub async fn list_random_questions(
        &self,
        nomor_kartu: &str,
        tanggal: &str,
    ) -> Result<JknResponse> {
        let path = normalize_path(
            "/SEP/FingerPrint/randomquestion/faskesterdaftar/nokapst/:nomorKartu/tglsep/:tanggal",
            &[
                ("nomorKartu", Some(nomor_kartu.to_string())),
                ("tanggal", Some(tanggal.to_string())),
            ],
        )?;
        self.client
            .send(ServiceType::Vclaim, RequestOptions::get(path))
            .await
    }

    pub async fn list_random_questions_typed(
        &self,
        nomor_kartu: &str,
        tanggal: &str,
    ) -> Result<VclaimRandomQuestionResponse> {
        self.list_random_questions(nomor_kartu, tanggal)
            .await?
            .into_response()
    }

    pub async fn send_random_question_answers<T: Serialize>(&self, data: T) -> Result<JknResponse> {
        self.client
            .send(
                ServiceType::Vclaim,
                RequestOptions::post("/SEP/FingerPrint/randomanswer").data(wrap_t_sep(data))?,
            )
            .await
    }
}

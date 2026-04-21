use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReferensiResult {
    pub kode: String,
    pub nama: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReferensiList {
    pub list: Vec<ReferensiResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReferensiDiagnosaResponse {
    pub diagnosa: Vec<ReferensiResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReferensiPoliResponse {
    pub poli: Vec<ReferensiResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReferensiFaskesResponse {
    pub faskes: Vec<ReferensiResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReferensiDpjpResponse {
    pub list: Vec<ReferensiResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaCob {
    #[serde(rename = "nmAsuransi")]
    pub nm_asuransi: Option<String>,
    #[serde(rename = "noAsuransi")]
    pub no_asuransi: Option<String>,
    #[serde(rename = "tglTAT")]
    pub tgl_tat: Option<String>,
    #[serde(rename = "tglTMT")]
    pub tgl_tmt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaHakKelas {
    pub keterangan: String,
    pub kode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaInformasi {
    pub dinsos: Option<String>,
    #[serde(rename = "noSKTM")]
    pub no_sktm: Option<String>,
    #[serde(rename = "prolanisPRB")]
    pub prolanis_prb: Option<String>,
    #[serde(rename = "eSEP")]
    pub e_sep: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaJenisPeserta {
    pub keterangan: String,
    pub kode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaMr {
    #[serde(rename = "noMR")]
    pub no_mr: Option<String>,
    #[serde(rename = "noTelepon")]
    pub no_telepon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaProvider {
    #[serde(rename = "kdProvider")]
    pub kd_provider: String,
    #[serde(rename = "nmProvider")]
    pub nm_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaStatusPeserta {
    pub keterangan: String,
    pub kode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaUmur {
    #[serde(rename = "umurSaatPelayanan")]
    pub umur_saat_pelayanan: String,
    #[serde(rename = "umurSekarang")]
    pub umur_sekarang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaData {
    pub cob: VclaimPesertaCob,
    #[serde(rename = "hakKelas")]
    pub hak_kelas: VclaimPesertaHakKelas,
    pub informasi: VclaimPesertaInformasi,
    #[serde(rename = "jenisPeserta")]
    pub jenis_peserta: VclaimPesertaJenisPeserta,
    pub mr: VclaimPesertaMr,
    pub nama: String,
    pub nik: String,
    #[serde(rename = "noKartu")]
    pub no_kartu: String,
    pub pisa: String,
    #[serde(rename = "provUmum")]
    pub prov_umum: VclaimPesertaProvider,
    pub sex: String,
    #[serde(rename = "statusPeserta")]
    pub status_peserta: VclaimPesertaStatusPeserta,
    #[serde(rename = "tglCetakKartu")]
    pub tgl_cetak_kartu: String,
    #[serde(rename = "tglLahir")]
    pub tgl_lahir: String,
    #[serde(rename = "tglTAT")]
    pub tgl_tat: String,
    #[serde(rename = "tglTMT")]
    pub tgl_tmt: String,
    pub umur: VclaimPesertaUmur,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPesertaResponse {
    pub peserta: VclaimPesertaData,
}

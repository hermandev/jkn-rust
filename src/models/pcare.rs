use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareListResponse<T> {
    pub count: u32,
    pub list: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareAlergiItem {
    #[serde(rename = "kdAlergi")]
    pub kd_alergi: String,
    #[serde(rename = "nmAlergi")]
    pub nm_alergi: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareDiagnosaItem {
    #[serde(rename = "kdDiag")]
    pub kd_diag: String,
    #[serde(rename = "nmDiag")]
    pub nm_diag: String,
    #[serde(rename = "nonSpesialis")]
    pub non_spesialis: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareDokterItem {
    #[serde(rename = "kdDokter")]
    pub kd_dokter: String,
    #[serde(rename = "nmDokter")]
    pub nm_dokter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareKesadaranItem {
    #[serde(rename = "kdSadar")]
    pub kd_sadar: String,
    #[serde(rename = "nmSadar")]
    pub nm_sadar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcarePoliItem {
    #[serde(rename = "kdPoli")]
    pub kd_poli: String,
    #[serde(rename = "nmPoli")]
    pub nm_poli: String,
    #[serde(rename = "poliSakit")]
    pub poli_sakit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcarePrognosaItem {
    #[serde(rename = "kdPrognosa")]
    pub kd_prognosa: String,
    #[serde(rename = "nmPrognosa")]
    pub nm_prognosa: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareStatusPulangItem {
    #[serde(rename = "kdStatusPulang")]
    pub kd_status_pulang: String,
    #[serde(rename = "nmStatusPulang")]
    pub nm_status_pulang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareProviderItem {
    #[serde(rename = "kdProvider")]
    pub kd_provider: String,
    #[serde(rename = "nmProvider")]
    pub nm_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcarePesertaProvider {
    #[serde(rename = "kdProvider")]
    pub kd_provider: Option<String>,
    #[serde(rename = "nmProvider")]
    pub nm_provider: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcarePesertaJenis {
    pub nama: String,
    pub kode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcarePesertaAsuransi {
    #[serde(rename = "kdAsuransi")]
    pub kd_asuransi: Option<String>,
    #[serde(rename = "nmAsuransi")]
    pub nm_asuransi: Option<String>,
    #[serde(rename = "noAsuransi")]
    pub no_asuransi: Option<String>,
    pub cob: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcarePesertaData {
    #[serde(rename = "noKartu")]
    pub no_kartu: String,
    pub nama: String,
    #[serde(rename = "hubunganKeluarga")]
    pub hubungan_keluarga: String,
    pub sex: String,
    #[serde(rename = "tglLahir")]
    pub tgl_lahir: String,
    #[serde(rename = "tglMulaiAktif")]
    pub tgl_mulai_aktif: String,
    #[serde(rename = "tglAkhirBerlaku")]
    pub tgl_akhir_berlaku: String,
    #[serde(rename = "kdProviderPst")]
    pub kd_provider_pst: PcarePesertaProvider,
    #[serde(rename = "kdProviderGigi")]
    pub kd_provider_gigi: PcarePesertaProvider,
    #[serde(rename = "jnsKelas")]
    pub jns_kelas: PcarePesertaJenis,
    #[serde(rename = "jnsPeserta")]
    pub jns_peserta: PcarePesertaJenis,
    #[serde(rename = "golDarah")]
    pub gol_darah: String,
    #[serde(rename = "noHP")]
    pub no_hp: String,
    #[serde(rename = "noKTP")]
    pub no_ktp: String,
    #[serde(rename = "pstProl")]
    pub pst_prol: String,
    #[serde(rename = "pstPrb")]
    pub pst_prb: String,
    pub aktif: bool,
    #[serde(rename = "ketAktif")]
    pub ket_aktif: String,
    pub asuransi: PcarePesertaAsuransi,
    pub tunggakan: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareSpesialisItem {
    #[serde(rename = "kdSpesialis")]
    pub kd_spesialis: String,
    #[serde(rename = "nmSpesialis")]
    pub nm_spesialis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareSubSpesialisItem {
    #[serde(rename = "kdSubSpesialis")]
    pub kd_sub_spesialis: String,
    #[serde(rename = "nmSubSpesialis")]
    pub nm_sub_spesialis: String,
    #[serde(rename = "kdPoliRujuk")]
    pub kd_poli_rujuk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareSaranaItem {
    #[serde(rename = "kdSarana")]
    pub kd_sarana: String,
    #[serde(rename = "nmSarana")]
    pub nm_sarana: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareKhususItem {
    #[serde(rename = "kdKhusus")]
    pub kd_khusus: String,
    #[serde(rename = "nmKhusus")]
    pub nm_khusus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareFaskesRujukanSubSpesialisItem {
    pub kdppk: String,
    pub nmppk: String,
    #[serde(rename = "alamatPpk")]
    pub alamat_ppk: String,
    #[serde(rename = "telpPpk")]
    pub telp_ppk: String,
    pub kelas: String,
    pub nmkc: String,
    pub distance: f64,
    pub jadwal: String,
    #[serde(rename = "jmlRujuk")]
    pub jml_rujuk: u32,
    pub kapasitas: u32,
    pub persentase: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRujukanPpkDati {
    #[serde(rename = "kdProp")]
    pub kd_prop: Option<String>,
    #[serde(rename = "kdDati")]
    pub kd_dati: String,
    #[serde(rename = "nmDati")]
    pub nm_dati: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRujukanPpkKr {
    #[serde(rename = "kdKR")]
    pub kd_kr: String,
    #[serde(rename = "nmKR")]
    pub nm_kr: String,
    pub alamat: Option<String>,
    pub telp: Option<String>,
    pub fax: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRujukanPpkKc {
    #[serde(rename = "kdKC")]
    pub kd_kc: String,
    #[serde(rename = "nmKC")]
    pub nm_kc: String,
    pub alamat: Option<String>,
    pub telp: Option<String>,
    pub fax: Option<String>,
    pub dati: PcareRujukanPpkDati,
    #[serde(rename = "kdKR")]
    pub kd_kr: PcareRujukanPpkKr,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRujukanPpk {
    #[serde(rename = "kdPPK")]
    pub kd_ppk: String,
    #[serde(rename = "nmPPK")]
    pub nm_ppk: String,
    pub alamat: Option<String>,
    pub kc: PcareRujukanPpkKc,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRujukanPoli {
    #[serde(rename = "kdPoli")]
    pub kd_poli: String,
    #[serde(rename = "nmPoli")]
    pub nm_poli: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRujukanDiagnosa {
    #[serde(rename = "kdDiag")]
    pub kd_diag: String,
    #[serde(rename = "nmDiag")]
    pub nm_diag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRujukanDokter {
    #[serde(rename = "kdDokter")]
    pub kd_dokter: String,
    #[serde(rename = "nmDokter")]
    pub nm_dokter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRujukanTacc {
    #[serde(rename = "nmTacc")]
    pub nm_tacc: Option<String>,
    #[serde(rename = "alasanTacc")]
    pub alasan_tacc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRujukanResult {
    #[serde(rename = "noRujukan")]
    pub no_rujukan: String,
    pub ppk: PcareRujukanPpk,
    #[serde(rename = "tglKunjungan")]
    pub tgl_kunjungan: String,
    pub poli: PcareRujukanPoli,
    #[serde(rename = "nokaPst")]
    pub noka_pst: String,
    #[serde(rename = "nmPst")]
    pub nm_pst: String,
    #[serde(rename = "tglLahir")]
    pub tgl_lahir: String,
    pub pisa: String,
    #[serde(rename = "ketPisa")]
    pub ket_pisa: String,
    pub sex: String,
    pub diag1: PcareRujukanDiagnosa,
    pub diag2: Option<String>,
    pub diag3: Option<String>,
    pub catatan: String,
    pub dokter: PcareRujukanDokter,
    pub tacc: PcareRujukanTacc,
    #[serde(rename = "infoDenda")]
    pub info_denda: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRiwayatProviderPelayanan {
    #[serde(rename = "kdProvider")]
    pub kd_provider: String,
    #[serde(rename = "nmProvider")]
    pub nm_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRiwayatPeserta {
    #[serde(rename = "noKartu")]
    pub no_kartu: String,
    pub nama: Option<String>,
    #[serde(rename = "hubunganKeluarga")]
    pub hubungan_keluarga: String,
    pub sex: Option<String>,
    #[serde(rename = "tglLahir")]
    pub tgl_lahir: Option<String>,
    #[serde(rename = "tglMulaiAktif")]
    pub tgl_mulai_aktif: Option<String>,
    #[serde(rename = "tglAkhirBerlaku")]
    pub tgl_akhir_berlaku: Option<String>,
    #[serde(rename = "kdPpkPst")]
    pub kd_ppk_pst: Option<String>,
    #[serde(rename = "kdPpkGigi")]
    pub kd_ppk_gigi: Option<String>,
    #[serde(rename = "jnsKelas")]
    pub jns_kelas: Option<String>,
    #[serde(rename = "jnsPeserta")]
    pub jns_peserta: Option<String>,
    #[serde(rename = "golDarah")]
    pub gol_darah: Option<String>,
    #[serde(rename = "noHP")]
    pub no_hp: Option<String>,
    #[serde(rename = "noKTP")]
    pub no_ktp: Option<String>,
    pub asuransi: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRiwayatNamedValue {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareRiwayatKunjunganItem {
    #[serde(rename = "noKunjungan")]
    pub no_kunjungan: String,
    #[serde(rename = "tglKunjungan")]
    pub tgl_kunjungan: String,
    #[serde(rename = "providerPelayanan")]
    pub provider_pelayanan: PcareRiwayatProviderPelayanan,
    pub peserta: PcareRiwayatPeserta,
    pub poli: PcareRiwayatNamedValue,
    #[serde(rename = "progProlanis")]
    pub prog_prolanis: PcareRiwayatNamedValue,
    pub keluhan: String,
    pub diagnosa1: PcareRiwayatNamedValue,
    pub diagnosa2: PcareRiwayatNamedValue,
    pub diagnosa3: PcareRiwayatNamedValue,
    pub kesadaran: PcareRiwayatNamedValue,
    pub sistole: i64,
    pub diastole: i64,
    #[serde(rename = "beratBadan")]
    pub berat_badan: i64,
    #[serde(rename = "tinggiBadan")]
    pub tinggi_badan: i64,
    #[serde(rename = "respRate")]
    pub resp_rate: i64,
    #[serde(rename = "heartRate")]
    pub heart_rate: i64,
    pub catatan: String,
    #[serde(rename = "rujukBalik")]
    pub rujuk_balik: i64,
    #[serde(rename = "providerAsalRujuk")]
    pub provider_asal_rujuk: PcareRiwayatNamedValue,
    #[serde(rename = "providerRujukLanjut")]
    pub provider_rujuk_lanjut: PcareRiwayatNamedValue,
    #[serde(rename = "pemFisikLain")]
    pub pem_fisik_lain: String,
    pub dokter: PcareRiwayatNamedValue,
    #[serde(rename = "statusPulang")]
    pub status_pulang: PcareRiwayatNamedValue,
    pub tkp: PcareRiwayatNamedValue,
    #[serde(rename = "poliRujukInternal")]
    pub poli_rujuk_internal: PcareRiwayatNamedValue,
    #[serde(rename = "poliRujukLanjut")]
    pub poli_rujuk_lanjut: PcareRiwayatNamedValue,
    #[serde(rename = "tglPulang")]
    pub tgl_pulang: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcareTKPItem {
    #[serde(rename = "kdTkp")]
    pub kd_tkp: String,
    #[serde(rename = "nmTkp")]
    pub nm_tkp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PcarePendaftaranResult {
    #[serde(rename = "noUrut")]
    pub no_urut: String,
    pub tgldaftar: String,
    #[serde(rename = "providerPelayanan")]
    pub provider_pelayanan: PcareProviderItem,
    pub peserta: PcarePesertaData,
    pub poli: PcarePoliItem,
    pub keluhan: String,
    pub status: String,
    pub sistole: i64,
    pub diastole: i64,
    #[serde(rename = "beratBadan")]
    pub berat_badan: i64,
    #[serde(rename = "tinggiBadan")]
    pub tinggi_badan: i64,
    #[serde(rename = "respRate")]
    pub resp_rate: i64,
    #[serde(rename = "heartRate")]
    pub heart_rate: i64,
    pub tkp: PcareTKPItem,
}

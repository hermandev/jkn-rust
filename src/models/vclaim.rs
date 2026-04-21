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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanDiagnosa {
    pub kode: String,
    pub nama: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanPelayanan {
    pub kode: String,
    pub nama: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanPeserta {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanData {
    pub diagnosa: VclaimRujukanDiagnosa,
    pub keluhan: String,
    #[serde(rename = "noKunjungan")]
    pub no_kunjungan: String,
    pub pelayanan: VclaimRujukanPelayanan,
    pub peserta: VclaimRujukanPeserta,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanResponse {
    pub rujukan: VclaimRujukanData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanListResponse {
    pub rujukan: Vec<VclaimRujukanData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanKhususItem {
    pub idrujukan: String,
    pub norujukan: String,
    pub nokapst: String,
    pub nmpst: String,
    pub diagppk: String,
    pub tglrujukan_awal: String,
    pub tglrujukan_berakhir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanKhususListResponse {
    pub rujukan: Vec<VclaimRujukanKhususItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanSpesialistikItem {
    #[serde(rename = "kodeSpesialis")]
    pub kode_spesialis: String,
    #[serde(rename = "namaSpesialis")]
    pub nama_spesialis: String,
    pub kapasitas: String,
    #[serde(rename = "jumlahRujukan")]
    pub jumlah_rujukan: String,
    pub persentase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanSpesialistikListResponse {
    pub list: Vec<VclaimRujukanSpesialistikItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanSaranaItem {
    #[serde(rename = "kodeSarana")]
    pub kode_sarana: String,
    #[serde(rename = "namaSarana")]
    pub nama_sarana: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanSaranaListResponse {
    pub list: Vec<VclaimRujukanSaranaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanKeluarItem {
    #[serde(rename = "noRujukan")]
    pub no_rujukan: String,
    #[serde(rename = "tglRujukan")]
    pub tgl_rujukan: String,
    #[serde(rename = "jnsPelayanan")]
    pub jns_pelayanan: String,
    #[serde(rename = "noSep")]
    pub no_sep: String,
    #[serde(rename = "noKartu")]
    pub no_kartu: String,
    pub nama: String,
    #[serde(rename = "ppkDirujuk")]
    pub ppk_dirujuk: String,
    #[serde(rename = "namaPpkDirujuk")]
    pub nama_ppk_dirujuk: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanKeluarListResponse {
    pub list: Vec<VclaimRujukanKeluarItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRujukanKeluarDetail {
    pub rujukan: VclaimRujukanKeluarItem,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimJumlahSepResponse {
    #[serde(rename = "jumlahSEP")]
    pub jumlah_sep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSepProvider {
    #[serde(rename = "kdProvider")]
    pub kd_provider: String,
    #[serde(rename = "nmProvider")]
    pub nm_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSepDetailPeserta {
    pub asuransi: Option<String>,
    #[serde(rename = "hakKelas")]
    pub hak_kelas: String,
    #[serde(rename = "jnsPeserta")]
    pub jns_peserta: String,
    pub kelamin: String,
    pub nama: String,
    #[serde(rename = "noKartu")]
    pub no_kartu: String,
    #[serde(rename = "noMr")]
    pub no_mr: String,
    #[serde(rename = "tglLahir")]
    pub tgl_lahir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSepDetailSimpleCodeName {
    pub kode: String,
    pub nama: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSepDetailKlsRawat {
    #[serde(rename = "klsRawatHak")]
    pub kls_rawat_hak: String,
    #[serde(rename = "klsRawatNaik")]
    pub kls_rawat_naik: Option<String>,
    pub pembiayaan: Option<String>,
    #[serde(rename = "penanggungJawab")]
    pub penanggung_jawab: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSepDetailKontrol {
    #[serde(rename = "kdDokter")]
    pub kd_dokter: String,
    #[serde(rename = "nmDokter")]
    pub nm_dokter: String,
    #[serde(rename = "noSurat")]
    pub no_surat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSepDetailLokasiKejadian {
    #[serde(rename = "kdKab")]
    pub kd_kab: Option<String>,
    #[serde(rename = "kdKec")]
    pub kd_kec: Option<String>,
    #[serde(rename = "kdProp")]
    pub kd_prop: Option<String>,
    #[serde(rename = "ketKejadian")]
    pub ket_kejadian: Option<String>,
    pub lokasi: Option<String>,
    #[serde(rename = "tglKejadian")]
    pub tgl_kejadian: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSepDetail {
    #[serde(rename = "noSep")]
    pub no_sep: String,
    #[serde(rename = "tglSep")]
    pub tgl_sep: String,
    #[serde(rename = "jnsPelayanan")]
    pub jns_pelayanan: String,
    #[serde(rename = "kelasRawat")]
    pub kelas_rawat: String,
    pub diagnosa: String,
    #[serde(rename = "noRujukan")]
    pub no_rujukan: String,
    pub poli: String,
    #[serde(rename = "poliEksekutif")]
    pub poli_eksekutif: String,
    pub catatan: String,
    pub penjamin: Option<String>,
    #[serde(rename = "kdStatusKecelakaan")]
    pub kd_status_kecelakaan: String,
    #[serde(rename = "nmstatusKecelakaan")]
    pub nmstatus_kecelakaan: String,
    pub informasi: Option<String>,
    #[serde(rename = "lokasiKejadian")]
    pub lokasi_kejadian: VclaimSepDetailLokasiKejadian,
    pub dpjp: VclaimSepDetailSimpleCodeName,
    pub peserta: VclaimSepDetailPeserta,
    #[serde(rename = "klsRawat")]
    pub kls_rawat: VclaimSepDetailKlsRawat,
    pub kontrol: VclaimSepDetailKontrol,
    pub cob: String,
    pub katarak: String,
    #[serde(rename = "tujuanKunj")]
    pub tujuan_kunj: VclaimSepDetailSimpleCodeName,
    #[serde(rename = "flagProcedure")]
    pub flag_procedure: VclaimSepDetailSimpleCodeName,
    #[serde(rename = "kdPenunjang")]
    pub kd_penunjang: VclaimSepDetailSimpleCodeName,
    #[serde(rename = "assestmenPel")]
    pub assestmen_pel: VclaimSepDetailSimpleCodeName,
    #[serde(rename = "eSEP")]
    pub e_sep: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSepWriteSimpleResponse {
    pub sep: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSuplesiJasaRaharjaItem {
    #[serde(rename = "noRegister")]
    pub no_register: String,
    #[serde(rename = "noSep")]
    pub no_sep: String,
    #[serde(rename = "noSepAwal")]
    pub no_sep_awal: String,
    #[serde(rename = "noSuratJaminan")]
    pub no_surat_jaminan: String,
    #[serde(rename = "tglKejadian")]
    pub tgl_kejadian: String,
    #[serde(rename = "tglSep")]
    pub tgl_sep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimSuplesiJasaRaharjaResponse {
    pub jaminan: Vec<VclaimSuplesiJasaRaharjaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimIndukKecelakaanItem {
    #[serde(rename = "noSEP")]
    pub no_sep: String,
    #[serde(rename = "tglKejadian")]
    pub tgl_kejadian: String,
    #[serde(rename = "ppkPelSEP")]
    pub ppk_pel_sep: String,
    #[serde(rename = "kdProp")]
    pub kd_prop: String,
    #[serde(rename = "kdKab")]
    pub kd_kab: String,
    #[serde(rename = "kdKec")]
    pub kd_kec: String,
    #[serde(rename = "ketKejadian")]
    pub ket_kejadian: String,
    #[serde(rename = "noSEPSuplesi")]
    pub no_sep_suplesi: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimIndukKecelakaanResponse {
    pub list: Vec<VclaimIndukKecelakaanItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPersetujuanSepItem {
    #[serde(rename = "noKartu")]
    pub no_kartu: String,
    pub nama: String,
    pub tglsep: String,
    pub jnspelayanan: String,
    pub persetujuan: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimPersetujuanSepListResponse {
    pub list: Vec<VclaimPersetujuanSepItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimTanggalPulangItem {
    #[serde(rename = "noSep")]
    pub no_sep: String,
    #[serde(rename = "noSepUpdating")]
    pub no_sep_updating: String,
    #[serde(rename = "jnsPelayanan")]
    pub jns_pelayanan: String,
    #[serde(rename = "ppkTujuan")]
    pub ppk_tujuan: String,
    #[serde(rename = "noKartu")]
    pub no_kartu: String,
    pub nama: String,
    #[serde(rename = "tglSep")]
    pub tgl_sep: String,
    #[serde(rename = "tglPulang")]
    pub tgl_pulang: String,
    pub status: String,
    #[serde(rename = "tglMeninggal")]
    pub tgl_meninggal: String,
    #[serde(rename = "noSurat")]
    pub no_surat: String,
    pub keterangan: String,
    pub user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimTanggalPulangListResponse {
    pub list: Vec<VclaimTanggalPulangItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimInacbgPesertaSep {
    pub kelamin: String,
    #[serde(rename = "klsRawat")]
    pub kls_rawat: String,
    pub nama: String,
    #[serde(rename = "noKartuBpjs")]
    pub no_kartu_bpjs: String,
    #[serde(rename = "noMr")]
    pub no_mr: String,
    #[serde(rename = "noRujukan")]
    pub no_rujukan: String,
    #[serde(rename = "tglLahir")]
    pub tgl_lahir: String,
    #[serde(rename = "tglPelayanan")]
    pub tgl_pelayanan: String,
    #[serde(rename = "tktPelayanan")]
    pub tkt_pelayanan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimInacbgResponse {
    pub pesertasep: VclaimInacbgPesertaSep,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimInternalSepItem {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimInternalSepListResponse {
    pub count: String,
    pub list: Vec<VclaimInternalSepItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimFingerPrintStatusResponse {
    pub kode: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimFingerPrintPesertaItem {
    #[serde(rename = "noKartu")]
    pub no_kartu: String,
    #[serde(rename = "noSEP")]
    pub no_sep: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimFingerPrintPesertaListResponse {
    pub list: Vec<VclaimFingerPrintPesertaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRandomQuestionFaskesItem {
    pub kode: String,
    pub nama: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VclaimRandomQuestionResponse {
    pub faskes: Vec<VclaimRandomQuestionFaskesItem>,
}

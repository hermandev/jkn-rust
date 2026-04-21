use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekKodeNama {
    pub kode: String,
    pub nama: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekListResponse<T> {
    pub list: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekDphoItem {
    pub kodeobat: String,
    pub namaobat: String,
    pub prb: String,
    pub kronis: String,
    pub kemo: String,
    pub harga: String,
    pub restriksi: String,
    pub generik: String,
    pub aktif: Option<String>,
    pub sedia: String,
    pub stok: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekDphoResponse {
    pub list: Vec<ApotekDphoItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekPoliResponse {
    pub poli: Vec<ApotekKodeNama>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekFaskesResponse {
    pub faskes: Vec<ApotekKodeNama>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekSettingResponse {
    pub kode: String,
    pub namaapoteker: String,
    pub namakepala: String,
    pub jabatankepala: String,
    pub nipkepala: String,
    pub siup: String,
    pub alamat: String,
    pub kota: String,
    pub namaverifikator: String,
    pub nppverifikator: String,
    pub namapetugasapotek: String,
    pub nippetugasapotek: String,
    pub checkstock: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekSpesialistikResponse {
    pub list: Vec<ApotekKodeNama>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekObatReferensiItem {
    pub kode: String,
    pub nama: String,
    pub harga: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekObatReferensiResponse {
    pub list: Vec<ApotekObatReferensiItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekResepSimpanResponse {
    #[serde(rename = "noSep_Kunjungan")]
    pub no_sep_kunjungan: String,
    #[serde(rename = "noKartu")]
    pub no_kartu: String,
    pub nama: String,
    #[serde(rename = "faskesAsal")]
    pub faskes_asal: String,
    #[serde(rename = "noApotik")]
    pub no_apotik: String,
    #[serde(rename = "noResep")]
    pub no_resep: String,
    #[serde(rename = "tglResep")]
    pub tgl_resep: String,
    #[serde(rename = "kdJnsObat")]
    pub kd_jns_obat: String,
    #[serde(rename = "byTagRsp")]
    pub by_tag_rsp: String,
    #[serde(rename = "byVerRsp")]
    pub by_ver_rsp: String,
    #[serde(rename = "tglEntry")]
    pub tgl_entry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekDaftarResepItem {
    #[serde(rename = "NORESEP")]
    pub noresep: String,
    #[serde(rename = "NOAPOTIK")]
    pub noapotik: String,
    #[serde(rename = "NOSEP_KUNJUNGAN")]
    pub nosep_kunjungan: String,
    #[serde(rename = "NOKARTU")]
    pub nokartu: String,
    #[serde(rename = "NAMA")]
    pub nama: String,
    #[serde(rename = "TGLENTRY")]
    pub tglentry: String,
    #[serde(rename = "TGLRESEP")]
    pub tglresep: String,
    #[serde(rename = "TGLPELRSP")]
    pub tglpelrsp: String,
    #[serde(rename = "BYTAGRSP")]
    pub bytagrsp: String,
    #[serde(rename = "BYVERRSP")]
    pub byverrsp: String,
    #[serde(rename = "KDJNSOBAT")]
    pub kdjnsobat: String,
    #[serde(rename = "FASKESASAL")]
    pub faskesasal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekDaftarResepResponse {
    pub resep: ApotekDaftarResepItem,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekPelayananObatItem {
    pub kodeobat: String,
    pub namaobat: String,
    pub tipeobat: String,
    pub signa1: String,
    pub signa2: String,
    pub hari: String,
    pub permintaan: Option<String>,
    pub jumlah: String,
    pub harga: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekPelayananDetailSep {
    #[serde(rename = "noSepApotek")]
    pub no_sep_apotek: String,
    #[serde(rename = "noSepAsal")]
    pub no_sep_asal: String,
    pub noresep: String,
    pub nokartu: String,
    pub nmpst: String,
    pub kdjnsobat: String,
    pub nmjnsobat: String,
    pub tglpelayanan: String,
    pub listobat: ApotekPelayananObatItem,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekPelayananDaftarResponse {
    pub detailsep: ApotekPelayananDetailSep,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekRiwayatHistoryItem {
    pub nosjp: String,
    pub tglpelayanan: String,
    pub noresep: String,
    pub kodeobat: String,
    pub namaobat: String,
    pub jmlobat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekRiwayatPeserta {
    pub nokartu: String,
    pub namapeserta: String,
    pub tgllhr: String,
    pub history: Vec<ApotekRiwayatHistoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekRiwayatResponse {
    pub list: Vec<ApotekRiwayatPeserta>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekSepKunjunganResponse {
    #[serde(rename = "noSep")]
    pub no_sep: String,
    pub faskesasalresep: String,
    pub nmfaskesasalresep: String,
    pub nokartu: String,
    pub namapeserta: String,
    pub jnskelamin: String,
    pub tgllhr: String,
    pub pisat: String,
    pub kdjenispeserta: String,
    pub nmjenispeserta: String,
    pub kodebu: String,
    pub namabu: String,
    pub tglsep: String,
    pub tglplgsep: String,
    pub jnspelayanan: String,
    pub nmdiag: String,
    pub poli: String,
    pub flagprb: String,
    pub namaprb: String,
    pub kodedokter: String,
    pub namadokter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekMonitoringListSepItem {
    pub nosepapotek: String,
    pub nosepaasal: String,
    pub nokartu: String,
    pub namapeserta: String,
    pub noresep: String,
    pub jnsobat: String,
    pub tglpelayanan: String,
    pub biayapengajuan: String,
    pub biayasetuju: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekMonitoringRekap {
    pub jumlahdata: String,
    pub totalbiayapengajuan: String,
    pub totalbiayasetuju: String,
    pub listsep: Vec<ApotekMonitoringListSepItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekMonitoringResponse {
    pub rekap: ApotekMonitoringRekap,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekPrbPesertaItem {
    #[serde(rename = "No")]
    pub no: i64,
    #[serde(rename = "NamaPeserta")]
    pub nama_peserta: String,
    #[serde(rename = "NomorKaPst")]
    pub nomor_ka_pst: String,
    #[serde(rename = "Alamat")]
    pub alamat: String,
    #[serde(rename = "TglSRB")]
    pub tgl_srb: String,
    #[serde(rename = "Diagnosa")]
    pub diagnosa: String,
    #[serde(rename = "Obat")]
    pub obat: String,
    #[serde(rename = "DPJP")]
    pub dpjp: String,
    #[serde(rename = "AsalFaskes")]
    pub asal_faskes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApotekPrbRekapPesertaResponse {
    pub list: Vec<ApotekPrbPesertaItem>,
}

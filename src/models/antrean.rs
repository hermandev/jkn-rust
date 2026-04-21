use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanRefPoliItem {
    pub nmpoli: String,
    pub nmsubspesialis: String,
    pub kdsubspesialis: String,
    pub kdpoli: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanRefDokterItem {
    pub namadokter: String,
    pub kodedokter: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanRefJadwalDokterItem {
    pub kodesubspesialis: String,
    pub hari: u32,
    pub kapasitaspasien: u32,
    pub libur: u32,
    pub namahari: String,
    pub jadwal: String,
    pub namasubspesialis: String,
    pub namadokter: String,
    pub kodepoli: String,
    pub namapoli: String,
    pub kodedokter: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanRefPoliFpItem {
    pub kodesubspesialis: String,
    pub namasubspesialis: String,
    pub kodepoli: String,
    pub namapoli: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanRefPasienFp {
    pub nomorkartu: String,
    pub nik: String,
    pub tgllahir: String,
    pub daftarfp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanTambahRequest {
    pub kodebooking: String,
    pub jenispasien: String,
    pub nomorkartu: String,
    pub nik: String,
    pub nohp: String,
    pub kodepoli: String,
    pub namapoli: String,
    pub pasienbaru: u32,
    pub norm: String,
    pub tanggalperiksa: String,
    pub kodedokter: String,
    pub namadokter: String,
    pub jampraktek: String,
    pub jeniskunjungan: u32,
    pub nomorreferensi: String,
    pub nomorantrean: String,
    pub angkaantrean: u32,
    pub estimasidilayani: u64,
    pub sisakuotajkn: u32,
    pub kuotajkn: u32,
    pub sisakuotanonjkn: u32,
    pub kuotanonjkn: u32,
    pub keterangan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanTambahFarmasiRequest {
    pub kodebooking: String,
    pub jenisresep: String,
    pub nomorantrean: u32,
    pub keterangan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanUpdateWaktuRequest {
    pub kodebooking: String,
    pub taskid: u32,
    pub waktu: u64,
    pub jenisresep: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanBatalRequest {
    pub kodebooking: String,
    pub keterangan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanTaskItem {
    pub wakturs: String,
    pub waktu: String,
    pub taskname: String,
    pub taskid: u32,
    pub kodebooking: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanDashboard {
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanDashboardList {
    pub list: Vec<AntreanDashboard>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntreanDetail {
    pub kodebooking: String,
    pub tanggal: String,
    pub kodepoli: String,
    pub kodedokter: u32,
    pub jampraktek: String,
    pub nik: String,
    pub nokapst: String,
    pub nohp: String,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

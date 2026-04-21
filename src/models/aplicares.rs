use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AplicaresBedData {
    pub kodekelas: String,
    pub koderuang: String,
    pub namaruang: String,
    pub kapasitas: u32,
    pub tersedia: u32,
    pub tersediapria: Option<u32>,
    pub tersediawanita: Option<u32>,
    pub tersediapriawanita: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AplicaresRefKamarItem {
    pub kodekelas: String,
    pub namakelas: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AplicaresRefKamarResponse {
    pub list: Vec<AplicaresRefKamarItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AplicaresReadBedData {
    pub kodeppk: String,
    pub rownumber: u32,
    pub lastupdate: String,
    #[serde(flatten)]
    pub bed: AplicaresBedData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AplicaresReadResponse {
    pub list: Vec<AplicaresReadBedData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AplicaresDeleteRequest {
    pub kodekelas: String,
    pub koderuang: String,
}

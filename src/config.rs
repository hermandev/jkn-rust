use std::collections::HashMap;
use std::env;

use serde::{Deserialize, Serialize};

use crate::error::{JknError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    Development,
    Production,
}

impl Mode {
    pub fn from_env() -> Self {
        match env::var("NODE_ENV").ok().as_deref() {
            Some("production") => Self::Production,
            _ => Self::Development,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    Aplicares,
    Vclaim,
    Antrean,
    Apotek,
    Pcare,
    Icare,
    RekamMedis,
}

impl ServiceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Aplicares => "aplicares",
            Self::Vclaim => "vclaim",
            Self::Antrean => "antrean",
            Self::Apotek => "apotek",
            Self::Pcare => "pcare",
            Self::Icare => "icare",
            Self::RekamMedis => "rekamMedis",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceBaseUrls {
    pub development: String,
    pub production: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ppk_code: String,
    pub cons_id: String,
    pub cons_secret: String,
    pub aplicares_user_key: Option<String>,
    pub vclaim_user_key: String,
    pub antrean_user_key: String,
    pub apotek_user_key: Option<String>,
    pub pcare_user_key: String,
    pub pcare_user_pass: String,
    pub icare_user_key: Option<String>,
    pub rekam_medis_user_key: Option<String>,
    pub mode: Mode,
    pub base_urls: HashMap<ServiceType, ServiceBaseUrls>,
}

impl Default for Config {
    fn default() -> Self {
        dotenv::dotenv().ok();

        Self {
            ppk_code: env::var("JKN_PPK_CODE").unwrap_or_default(),
            cons_id: env::var("JKN_CONS_ID").unwrap_or_default(),
            cons_secret: env::var("JKN_CONS_SECRET").unwrap_or_default(),
            aplicares_user_key: env::var("JKN_APLICARES_USER_KEY").ok(),
            vclaim_user_key: env::var("JKN_VCLAIM_USER_KEY").unwrap_or_default(),
            antrean_user_key: env::var("JKN_ANTREAN_USER_KEY").unwrap_or_default(),
            apotek_user_key: env::var("JKN_APOTEK_USER_KEY").ok(),
            pcare_user_key: env::var("JKN_PCARE_USER_KEY").unwrap_or_default(),
            pcare_user_pass: env::var("JKN_PCARE_USER_PASS").unwrap_or_default(),
            icare_user_key: env::var("JKN_ICARE_USER_KEY").ok(),
            rekam_medis_user_key: env::var("JKN_REKAM_MEDIS_USER_KEY").ok(),
            mode: Mode::from_env(),
            base_urls: default_base_urls(),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Self::default().validate()
    }

    pub fn validate(mut self) -> Result<Self> {
        if self.cons_id.is_empty() || self.cons_secret.is_empty() {
            return Err(JknError::Config(
                "cons id and secret are not defined".to_string(),
            ));
        }

        if self
            .aplicares_user_key
            .as_deref()
            .is_none_or(|value| value.is_empty())
        {
            self.aplicares_user_key = Some(self.vclaim_user_key.clone());
        }
        if self
            .apotek_user_key
            .as_deref()
            .is_none_or(|value| value.is_empty())
        {
            self.apotek_user_key = Some(self.vclaim_user_key.clone());
        }
        if self
            .icare_user_key
            .as_deref()
            .is_none_or(|value| value.is_empty())
        {
            self.icare_user_key = Some(self.vclaim_user_key.clone());
        }
        if self
            .rekam_medis_user_key
            .as_deref()
            .is_none_or(|value| value.is_empty())
        {
            self.rekam_medis_user_key = Some(self.vclaim_user_key.clone());
        }

        Ok(self)
    }

    pub fn base_url(&self, service: ServiceType) -> Result<&str> {
        let urls = self.base_urls.get(&service).ok_or_else(|| {
            JknError::Config(format!("missing base url config for {}", service.as_str()))
        })?;

        Ok(match self.mode {
            Mode::Development => urls.development.as_str(),
            Mode::Production => urls.production.as_str(),
        })
    }

    pub fn user_key(&self, service: ServiceType) -> Result<&str> {
        let key = match service {
            ServiceType::Aplicares => self.aplicares_user_key.as_deref(),
            ServiceType::Vclaim => Some(self.vclaim_user_key.as_str()),
            ServiceType::Antrean => Some(self.antrean_user_key.as_str()),
            ServiceType::Apotek => self.apotek_user_key.as_deref(),
            ServiceType::Pcare => Some(self.pcare_user_key.as_str()),
            ServiceType::Icare => self.icare_user_key.as_deref(),
            ServiceType::RekamMedis => self.rekam_medis_user_key.as_deref(),
        };

        key.ok_or_else(|| {
            JknError::Config(format!(
                "failed to get user key of type {}",
                service.as_str()
            ))
        })
    }

    pub fn require_ppk_code(&self) -> Result<&str> {
        if self.ppk_code.is_empty() {
            return Err(JknError::Config(
                "the \"ppk_code\" config value must not be empty".to_string(),
            ));
        }
        Ok(self.ppk_code.as_str())
    }
}

pub fn default_base_urls() -> HashMap<ServiceType, ServiceBaseUrls> {
    HashMap::from([
        (
            ServiceType::Aplicares,
            ServiceBaseUrls {
                development: "https://dvlp.bpjs-kesehatan.go.id:8888/aplicaresws".to_string(),
                production: "https://new-api.bpjs-kesehatan.go.id/aplicaresws".to_string(),
            },
        ),
        (
            ServiceType::Vclaim,
            ServiceBaseUrls {
                development: "https://apijkn-dev.bpjs-kesehatan.go.id/vclaim-rest-dev".to_string(),
                production: "https://apijkn.bpjs-kesehatan.go.id/vclaim-rest".to_string(),
            },
        ),
        (
            ServiceType::Antrean,
            ServiceBaseUrls {
                development: "https://apijkn-dev.bpjs-kesehatan.go.id/antreanrs_dev".to_string(),
                production: "https://apijkn.bpjs-kesehatan.go.id/antreanrs".to_string(),
            },
        ),
        (
            ServiceType::Apotek,
            ServiceBaseUrls {
                development: "https://apijkn-dev.bpjs-kesehatan.go.id/apotek-rest-dev".to_string(),
                production: "https://apijkn.bpjs-kesehatan.go.id/apotek-rest".to_string(),
            },
        ),
        (
            ServiceType::Pcare,
            ServiceBaseUrls {
                development: "https://apijkn-dev.bpjs-kesehatan.go.id/pcare-rest-dev".to_string(),
                production: "https://apijkn.bpjs-kesehatan.go.id/pcare-rest".to_string(),
            },
        ),
        (
            ServiceType::Icare,
            ServiceBaseUrls {
                development: "https://apijkn-dev.bpjs-kesehatan.go.id/ihs_dev".to_string(),
                production: "https://apijkn.bpjs-kesehatan.go.id/wsihs".to_string(),
            },
        ),
        (
            ServiceType::RekamMedis,
            ServiceBaseUrls {
                development: "https://apijkn-dev.bpjs-kesehatan.go.id/erekammedis_dev".to_string(),
                production: "https://apijkn.bpjs-kesehatan.go.id/medicalrecord".to_string(),
            },
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fallback_user_keys_follow_vclaim() {
        let config = Config {
            cons_id: "x".into(),
            cons_secret: "y".into(),
            vclaim_user_key: "vk".into(),
            antrean_user_key: "ak".into(),
            pcare_user_key: "pk".into(),
            pcare_user_pass: "u:p".into(),
            ..Default::default()
        }
        .validate()
        .expect("config should validate");

        assert_eq!(config.apotek_user_key.as_deref(), Some("vk"));
        assert_eq!(config.aplicares_user_key.as_deref(), Some("vk"));
        assert_eq!(config.icare_user_key.as_deref(), Some("vk"));
        assert_eq!(config.rekam_medis_user_key.as_deref(), Some("vk"));
    }
}

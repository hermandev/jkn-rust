pub mod antrean;
pub mod antrean_fktp;
pub mod aplicares;
pub mod apotek;
pub mod icare;
pub mod pcare;
pub mod rekam_medis;
pub mod vclaim;

use std::sync::Arc;

use crate::JknClient;
use crate::error::Result;

#[derive(Clone)]
pub struct Jkn {
    client: Arc<JknClient>,
}

impl Jkn {
    pub fn new(client: JknClient) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    pub fn from_env() -> Result<Self> {
        Ok(Self::new(JknClient::from_env()?))
    }

    pub fn client(&self) -> Arc<JknClient> {
        Arc::clone(&self.client)
    }

    pub fn aplicares(&self) -> aplicares::Aplicares {
        aplicares::Aplicares::new(self.client())
    }

    pub fn antrean(&self) -> antrean::Antrean {
        antrean::Antrean::new(self.client())
    }

    pub fn antrean_fktp(&self) -> antrean_fktp::AntreanFktp {
        antrean_fktp::AntreanFktp::new(self.client())
    }

    pub fn apotek(&self) -> apotek::Apotek {
        apotek::Apotek::new(self.client())
    }

    pub fn icare(&self) -> icare::ICare {
        icare::ICare::new(self.client())
    }

    pub fn pcare(&self) -> pcare::PCare {
        pcare::PCare::new(self.client())
    }

    pub fn rekam_medis(&self) -> rekam_medis::RekamMedis {
        rekam_medis::RekamMedis::new(self.client())
    }

    pub fn vclaim(&self) -> vclaim::VClaim {
        vclaim::VClaim::new(self.client())
    }
}

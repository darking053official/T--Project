use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThunderConfig {
    pub homepage: String,
    pub telemetry: bool,
}

impl Default for ThunderConfig {
    fn default() -> Self {
        Self {
            homepage: "about:blank".into(),
            telemetry: false,
        }
    }
}

pub struct ThunderCore {
    cfg: ThunderConfig,
}

impl ThunderCore {
    pub fn new(cfg: ThunderConfig) -> Self {
        Self { cfg }
    }

    pub fn init(&self) -> Result<()> {
        log::info!("ThunderCore initialized (telemetry={})", self.cfg.telemetry);
        Ok(())
    }

    pub fn homepage(&self) -> &str {
        &self.cfg.homepage
    }

    pub fn navigate(&self, url: &str) -> Result<()> {
        // TODO: Gecko FFI çağrısı
        log::info!("Navigate -> {}", url);
        Ok(())
    }
}

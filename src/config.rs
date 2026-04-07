use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::RouterResult;
use crate::fs::ensure_ever_home_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EverConfig {
    #[serde(default)]
    pub global: GlobalConfig,
    #[serde(default)]
    pub auth: AuthConfig,
    #[serde(default)]
    pub registry: RegistryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default = "default_false")]
    pub telemetry: bool,
    #[serde(default = "default_true")]
    pub auto_update_check: bool,
    #[serde(default)]
    pub default_org: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    #[serde(default)]
    pub api_token: Option<String>,
    #[serde(default)]
    pub api_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    #[serde(default = "default_npm_prefix")]
    pub npm_prefix: String,
}

impl Default for EverConfig {
    fn default() -> Self {
        Self {
            global: GlobalConfig::default(),
            auth: AuthConfig::default(),
            registry: RegistryConfig::default(),
        }
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            telemetry: default_false(),
            auto_update_check: default_true(),
            default_org: None,
        }
    }
}

impl Default for RegistryConfig {
    fn default() -> Self {
        Self {
            npm_prefix: default_npm_prefix(),
        }
    }
}

impl EverConfig {
    pub fn load_or_default() -> RouterResult<Self> {
        let path = config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let data = fs::read_to_string(path)?;
        Ok(toml::from_str(&data)?)
    }

    pub fn save(&self) -> RouterResult<()> {
        let path = config_path()?;
        let data = toml::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}

pub fn config_path() -> RouterResult<PathBuf> {
    let home = ensure_ever_home_dir()?;
    Ok(home.join("config.toml"))
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_npm_prefix() -> String {
    "ever".to_string()
}

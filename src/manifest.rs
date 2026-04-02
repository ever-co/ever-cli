use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::error::RouterResult;
use crate::fs::ensure_ever_home_dir;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PluginManifest {
    pub version: u32,
    pub plugins: BTreeMap<String, PluginEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEntry {
    pub binary: PathBuf,
    pub package: Option<String>,
    pub source: Option<String>,
    pub version: Option<String>,
    pub installed_at: Option<String>,
}

impl PluginEntry {
    pub fn new(binary: PathBuf, package: Option<String>, source: Option<String>) -> Self {
        Self {
            binary,
            package,
            source,
            version: None,
            installed_at: None,
        }
    }
}

impl PluginManifest {
    pub fn load_or_default() -> RouterResult<Self> {
        let path = manifest_path()?;

        if !path.exists() {
            return Ok(Self {
                version: 1,
                plugins: BTreeMap::new(),
            });
        }

        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save(&self) -> RouterResult<()> {
        let path = manifest_path()?;
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn get(&self, product: &str) -> Option<&PluginEntry> {
        self.plugins.get(product)
    }

    pub fn upsert(&mut self, product: String, entry: PluginEntry) {
        self.plugins.insert(product, entry);
    }
}

pub fn manifest_path() -> RouterResult<PathBuf> {
    let home = ensure_ever_home_dir()?;
    Ok(home.join("plugins.json"))
}

use std::path::{Path, PathBuf};

use crate::catalog;
use crate::error::{RouterError, RouterResult};
use crate::manifest::{PluginEntry, PluginManifest};

pub fn resolve_binary_name(product: &str) -> String {
    catalog::binary_name(product)
}

pub fn route(product: String, args: Vec<String>) -> RouterResult<()> {
    let binary = resolve_product_binary(&product)?;

    crate::exec::exec_binary(binary, args)
}

pub fn resolve_from_path(product: &str) -> Option<PathBuf> {
    which::which(resolve_binary_name(product)).ok()
}

pub fn resolve_product_binary(product: &str) -> RouterResult<PathBuf> {
    let mut manifest = PluginManifest::load_or_default()?;

    if let Some(entry) = manifest.get(product) {
        if path_exists(&entry.binary) {
            return Ok(entry.binary.clone());
        }
    }

    if let Some(binary) = resolve_from_path(product) {
        let package = catalog::find(product).map(|entry| entry.npm_package.to_string());
        manifest.upsert(
            product.to_string(),
            PluginEntry::new(binary.clone(), package, Some("path".to_string())),
        );
        manifest.save()?;
        return Ok(binary);
    }

    Err(RouterError::ProductNotInstalled {
        product: product.to_string(),
    })
}

fn path_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

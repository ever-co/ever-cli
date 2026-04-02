use crate::catalog::PRODUCT_CATALOG;
use crate::error::RouterResult;
use crate::manifest::PluginManifest;
use crate::resolver::resolve_from_path;

pub fn run() -> RouterResult<()> {
    let manifest = PluginManifest::load_or_default()?;

    println!("Known Ever products:");
    for entry in PRODUCT_CATALOG {
        let status = if let Some(plugin) = manifest.get(entry.product) {
            if plugin.binary.exists() {
                format!("installed ({})", plugin.binary.display())
            } else if let Some(binary) = resolve_from_path(entry.product) {
                format!("available on PATH ({})", binary.display())
            } else {
                "manifest entry is stale".to_string()
            }
        } else if let Some(binary) = resolve_from_path(entry.product) {
            format!("available on PATH ({})", binary.display())
        } else {
            "not installed".to_string()
        };

        println!("  - {:<12} {:<18} {}", entry.product, status, entry.npm_package);
    }
    Ok(())
}

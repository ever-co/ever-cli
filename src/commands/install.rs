use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::catalog;
use crate::error::{RouterError, RouterResult};
use crate::manifest::{PluginEntry, PluginManifest};
use crate::resolver::resolve_from_path;

pub fn run(product: String, source: Option<String>) -> RouterResult<()> {
    let requested_source = source.unwrap_or_else(|| "npm".to_string());
    if requested_source != "npm" {
        return Err(RouterError::Message(format!(
            "Install source '{requested_source}' is not implemented yet. Use --from npm or omit the flag."
        )));
    }

    let entry = catalog::find(&product)
        .ok_or_else(|| RouterError::Message(format!("Unknown product '{product}'. Run: ever list")))?;

    println!("Installing {} via npm...", entry.npm_package);
    let status = Command::new("npm")
        .args(["install", "-g", entry.npm_package])
        .status()?;

    if !status.success() {
        return Err(RouterError::Message(format!(
            "npm install failed for '{}'",
            entry.npm_package
        )));
    }

    let binary = resolve_from_path(&product).ok_or_else(|| {
        RouterError::Message(format!(
            "Installed '{}', but could not find '{}' on PATH",
            entry.npm_package,
            catalog::binary_name(&product)
        ))
    })?;

    let mut manifest = PluginManifest::load_or_default()?;
    let mut plugin = PluginEntry::new(
        binary.clone(),
        Some(entry.npm_package.to_string()),
        Some("npm".to_string()),
    );
    plugin.installed_at = Some(current_timestamp_string());
    manifest.upsert(product.clone(), plugin);
    manifest.save()?;

    println!("Registered {} at {}", product, binary.display());

    Ok(())
}

fn current_timestamp_string() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);

    seconds.to_string()
}

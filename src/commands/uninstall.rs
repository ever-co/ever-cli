use std::process::Command;

use crate::catalog;
use crate::error::{RouterError, RouterResult};
use crate::manifest::PluginManifest;

pub fn run(product: String) -> RouterResult<()> {
    let mut manifest = PluginManifest::load_or_default()?;
    let existing = manifest.get(&product).cloned();
    let package_name = existing
        .as_ref()
        .and_then(|entry| entry.package.clone())
        .or_else(|| catalog::find(&product).map(|entry| entry.npm_package.to_string()))
        .ok_or_else(|| RouterError::Message(format!("Unknown product '{product}'. Run: ever list")))?;

    println!("Uninstalling {}...", package_name);
    let status = Command::new("npm")
        .args(["uninstall", "-g", &package_name])
        .status()?;

    if !status.success() {
        return Err(RouterError::Message(format!(
            "npm uninstall failed for '{}'",
            package_name
        )));
    }

    manifest.remove(&product);
    manifest.save()?;

    println!("Removed {} from the manifest.", product);
    Ok(())
}

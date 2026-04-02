use crate::catalog;
use crate::error::{RouterError, RouterResult};
use crate::manifest::{PluginEntry, PluginManifest};
use crate::npm::{detect_global_package_version, install_global_package};
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
    install_global_package(entry.npm_package)?;

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
    plugin.version = detect_global_package_version(entry.npm_package)?;
    plugin.installed_at = Some(timestamp::iso8601_now()?);
    manifest.upsert(product.clone(), plugin);
    manifest.save()?;

    println!("Registered {} at {}", product, binary.display());

    Ok(())
}

mod timestamp {
    use chrono::Utc;
    use crate::error::{RouterError, RouterResult};

    pub fn iso8601_now() -> RouterResult<String> {
        let now = Utc::now();
        if now.timestamp() < 0 {
            return Err(RouterError::Message("Failed to format current timestamp".to_string()));
        }

        Ok(now.to_rfc3339())
    }
}

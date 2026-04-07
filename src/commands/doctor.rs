use crate::config::{config_path, EverConfig};
use crate::error::RouterResult;
use crate::manifest::{manifest_path, PluginManifest};
use crate::resolver::resolve_from_path;
use crate::fs::ever_home_dir;

pub fn run() -> RouterResult<()> {
    let home = ever_home_dir()?;
    let config_path = config_path()?;
    let path = manifest_path()?;
    let manifest = PluginManifest::load_or_default()?;
    let config = EverConfig::load_or_default()?;

    println!("ever doctor");
    println!("  home: {}", home.display());
    println!("  config: {}", config_path.display());
    println!("  manifest: {}", path.display());
    println!(
        "  auth token: {}",
        if config.auth.api_token.is_some() {
            "configured"
        } else {
            "not configured"
        }
    );
    println!("  registered plugins: {}", manifest.plugins.len());
    println!();

    if manifest.plugins.is_empty() {
        println!("No registered plugins yet.");
        return Ok(());
    }

    for (product, plugin) in &manifest.plugins {
        if plugin.binary.exists() {
            println!("  ✓ {:<12} {}", product, plugin.binary.display());
        } else if let Some(binary) = resolve_from_path(product) {
            println!(
                "  ⚠ {:<12} manifest path missing, but found on PATH at {}",
                product,
                binary.display()
            );
        } else {
            println!(
                "  ✗ {:<12} missing binary at {}",
                product,
                plugin.binary.display()
            );
        }
    }

    Ok(())
}

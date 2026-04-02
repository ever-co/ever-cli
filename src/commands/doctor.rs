use crate::error::RouterResult;
use crate::manifest::{manifest_path, PluginManifest};

pub fn run() -> RouterResult<()> {
    let path = manifest_path()?;
    let manifest = PluginManifest::load_or_default()?;

    println!("ever doctor");
    println!("  manifest: {}", path.display());
    println!("  registered plugins: {}", manifest.plugins.len());

    Ok(())
}

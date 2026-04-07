use crate::catalog;
use crate::error::{RouterError, RouterResult};
use crate::commands::install::timestamp;
use crate::manifest::{PluginEntry, PluginManifest};
use crate::npm::{detect_global_package_version, update_global_package, update_global_packages};
use crate::resolver::resolve_from_path;

pub fn run(product: Option<String>) -> RouterResult<()> {
    match product {
        Some(product) => update_single(product),
        None => update_all(),
    }
}

fn update_single(product: String) -> RouterResult<()> {
    let mut manifest = PluginManifest::load_or_default()?;
    let existing = manifest.get(&product).cloned();
    let package_name = existing
        .as_ref()
        .and_then(|entry| entry.package.clone())
        .or_else(|| catalog::find(&product).map(|entry| entry.npm_package.to_string()))
        .ok_or_else(|| RouterError::Message(format!("Unknown product '{product}'. Run: ever list")))?;

    println!("Updating {}...", package_name);
    update_global_package(&package_name)?;

    refresh_manifest_entry(&mut manifest, &product, package_name)?;
    manifest.save()?;
    println!("Updated {}.", product);
    Ok(())
}

fn update_all() -> RouterResult<()> {
    let mut manifest = PluginManifest::load_or_default()?;
    let installed_products: Vec<(String, String)> = manifest
        .plugins
        .iter()
        .filter_map(|(product, entry)| {
            if entry.source.as_deref() == Some("npm") {
                entry.package.clone().map(|package| (product.clone(), package))
            } else {
                None
            }
        })
        .collect();

    if installed_products.is_empty() {
        println!("No npm-installed products are registered in the manifest.");
        return Ok(());
    }

    let packages: Vec<&str> = installed_products.iter().map(|(_, package)| package.as_str()).collect();
    println!("Updating {} product(s)...", packages.len());
    update_global_packages(&packages)?;

    for (product, package) in installed_products {
        refresh_manifest_entry(&mut manifest, &product, package)?;
    }

    manifest.save()?;
    println!("Updated all registered npm products.");
    Ok(())
}

fn refresh_manifest_entry(
    manifest: &mut PluginManifest,
    product: &str,
    package_name: String,
) -> RouterResult<()> {
    let binary = resolve_from_path(product).ok_or_else(|| {
        RouterError::Message(format!(
            "Updated '{}', but could not find '{}' on PATH",
            package_name,
            catalog::binary_name(product)
        ))
    })?;

    let existing = manifest.get(product).cloned();
    let mut entry = existing.unwrap_or_else(|| {
        PluginEntry::new(binary.clone(), Some(package_name.clone()), Some("npm".to_string()))
    });
    entry.binary = binary;
    entry.package = Some(package_name.clone());
    entry.source = Some("npm".to_string());
    entry.version = detect_global_package_version(&package_name)?;
    entry.installed_at = Some(timestamp::iso8601_now()?);
    manifest.upsert(product.to_string(), entry);

    Ok(())
}

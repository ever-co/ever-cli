use std::process::Command;

use crate::error::{RouterError, RouterResult};

pub fn detect_global_package_version(package_name: &str) -> RouterResult<Option<String>> {
    let output = Command::new("npm")
        .args(["list", "-g", package_name, "--depth=0", "--json"])
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout)?;

    let version = parsed
        .get("dependencies")
        .and_then(|deps| deps.get(package_name))
        .and_then(|pkg| pkg.get("version"))
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());

    Ok(version)
}

pub fn install_global_package(package_name: &str) -> RouterResult<()> {
    let status = Command::new("npm")
        .args(["install", "-g", package_name])
        .status()?;

    if !status.success() {
        return Err(RouterError::Message(format!(
            "npm install failed for '{}'",
            package_name
        )));
    }

    Ok(())
}

pub fn update_global_package(package_name: &str) -> RouterResult<()> {
    let status = Command::new("npm")
        .args(["update", "-g", package_name])
        .status()?;

    if !status.success() {
        return Err(RouterError::Message(format!(
            "npm update failed for '{}'",
            package_name
        )));
    }

    Ok(())
}

pub fn update_global_packages(package_names: &[&str]) -> RouterResult<()> {
    let status = Command::new("npm")
        .args(["update", "-g"])
        .args(package_names)
        .status()?;

    if !status.success() {
        return Err(RouterError::Message(
            "npm update failed for one or more products".to_string(),
        ));
    }

    Ok(())
}

pub fn uninstall_global_package(package_name: &str) -> RouterResult<()> {
    let status = Command::new("npm")
        .args(["uninstall", "-g", package_name])
        .status()?;

    if !status.success() {
        return Err(RouterError::Message(format!(
            "npm uninstall failed for '{}'",
            package_name
        )));
    }

    Ok(())
}

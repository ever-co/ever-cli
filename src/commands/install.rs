use crate::catalog;
use crate::error::{RouterError, RouterResult};

pub fn run(product: String, source: Option<String>) -> RouterResult<()> {
    let entry = catalog::find(&product).ok_or_else(|| RouterError::Message(format!(
        "Unknown product '{product}'. Run: ever list"
    )))?;

    println!(
        "Install is not implemented yet. Planned source: {} | npm package: {}",
        source.unwrap_or_else(|| "npm".to_string()),
        entry.npm_package
    );

    Ok(())
}

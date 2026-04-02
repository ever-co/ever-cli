use std::path::PathBuf;

use crate::error::{RouterError, RouterResult};

pub fn resolve_binary_name(product: &str) -> String {
    format!("ever-{product}")
}

pub fn route(product: String, args: Vec<String>) -> RouterResult<()> {
    let binary = which::which(resolve_binary_name(&product))
        .map_err(|_| RouterError::ProductNotInstalled { product: product.clone() })?;

    crate::exec::exec_binary(binary, args)
}

pub fn resolve_from_path(product: &str) -> Option<PathBuf> {
    which::which(resolve_binary_name(product)).ok()
}

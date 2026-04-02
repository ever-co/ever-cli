use std::fs;
use std::path::PathBuf;

use crate::error::{RouterError, RouterResult};

pub fn ever_home_dir() -> RouterResult<PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| {
        RouterError::Message("Could not resolve the current user's home directory".to_string())
    })?;

    Ok(home.join(".ever"))
}

pub fn ensure_ever_home_dir() -> RouterResult<PathBuf> {
    let path = ever_home_dir()?;
    fs::create_dir_all(&path)?;
    Ok(path)
}

use std::process::Command;

use crate::error::RouterResult;

pub fn exec_binary(binary: std::path::PathBuf, args: Vec<String>) -> RouterResult<()> {
    let status = Command::new(binary).args(args).status()?;
    std::process::exit(status.code().unwrap_or(1));
}

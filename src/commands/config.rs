use crate::config::{config_path, EverConfig};
use crate::error::RouterResult;

pub fn run(args: Vec<String>) -> RouterResult<()> {
    let path = config_path()?;
    let config = EverConfig::load_or_default()?;

    if args.is_empty() {
        println!("Ever config file: {}", path.display());
        println!(
            "Auth token: {}",
            if config.auth.api_token.is_some() {
                "configured"
            } else {
                "not configured"
            }
        );
        return Ok(());
    }

    if args.len() == 1 && args[0] == "init" {
        config.save()?;
        println!("Initialized config at {}", path.display());
        return Ok(());
    }

    println!("Config command is partially implemented. Received: {}", args.join(" "));

    Ok(())
}

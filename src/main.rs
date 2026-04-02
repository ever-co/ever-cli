mod catalog;
mod cli;
mod commands;
mod config;
mod error;
mod exec;
mod fs;
mod manifest;
mod resolver;

use clap::Parser;

use crate::cli::{Cli, Command};
use crate::error::RouterResult;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> RouterResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Help => commands::help::run(),
        Command::Version => commands::version::run(),
        Command::List => commands::list::run(),
        Command::Doctor => commands::doctor::run(),
        Command::Install { product, source } => commands::install::run(product, source),
        Command::Uninstall { product } => commands::uninstall::run(product),
        Command::Update { product } => commands::update::run(product),
        Command::Config { args } => commands::config::run(args),
        route_command => {
            let (product, args) = route_command.into_route_parts().ok_or_else(|| {
                crate::error::RouterError::Message(
                    "Missing product name. Run 'ever help' for usage.".to_string(),
                )
            })?;

            resolver::route(product, args)
        }
    }
}

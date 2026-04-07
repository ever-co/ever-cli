use crate::error::RouterResult;

pub fn run() -> RouterResult<()> {
    println!("ever — Ever ecosystem CLI router");
    println!();
    println!("USAGE:");
    println!("  ever <product> <command> [args...]");
    println!("  ever install <product>");
    println!("  ever uninstall <product>");
    println!("  ever update [product]");
    println!("  ever list");
    println!("  ever doctor");
    println!("  ever config [args]");
    println!("  ever version");
    println!("  ever help");
    println!();
    println!("EXAMPLES:");
    println!("  ever works init");
    println!("  ever cloc start timer");
    println!("  ever os run agents --verbose");
    println!();
    println!("BUILT-IN COMMANDS:");
    println!("  install      Install a product CLI");
    println!("  uninstall    Remove a product CLI and its manifest entry");
    println!("  update       Update one or all registered npm-installed product CLIs");
    println!("  list         List known products and their install state");
    println!("  doctor       Check router configuration and manifest entries");
    println!("  config       Show or initialize ~/.ever/config.toml");
    println!("  version      Show the router version");
    println!("  help         Show this help output");
    Ok(())
}

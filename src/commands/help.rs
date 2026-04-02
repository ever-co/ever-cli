use crate::error::RouterResult;

pub fn run() -> RouterResult<()> {
    println!("ever — Ever ecosystem CLI router");
    println!();
    println!("USAGE:");
    println!("  ever <product> <command> [args...]");
    println!("  ever install <product>");
    println!("  ever list");
    println!("  ever doctor");
    Ok(())
}

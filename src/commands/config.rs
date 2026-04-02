use crate::error::RouterResult;

pub fn run(args: Vec<String>) -> RouterResult<()> {
    if args.is_empty() {
        println!("Config command is not implemented yet.");
    } else {
        println!("Config command is not implemented yet. Args: {}", args.join(" "));
    }

    Ok(())
}

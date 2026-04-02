use crate::error::RouterResult;

pub fn run() -> RouterResult<()> {
    println!("ever {}", env!("CARGO_PKG_VERSION"));
    Ok(())
}

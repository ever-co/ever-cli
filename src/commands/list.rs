use crate::catalog::PRODUCT_CATALOG;
use crate::error::RouterResult;

pub fn run() -> RouterResult<()> {
    println!("Known Ever products:");
    for entry in PRODUCT_CATALOG {
        println!("  - {} ({})", entry.product, entry.npm_package);
    }
    Ok(())
}

use crate::error::RouterResult;

pub fn run(product: Option<String>) -> RouterResult<()> {
    match product {
        Some(product) => println!("Update is not implemented yet for '{product}'."),
        None => println!("Update-all is not implemented yet."),
    }

    Ok(())
}

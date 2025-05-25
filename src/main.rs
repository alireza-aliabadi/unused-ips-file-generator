use std::error::Error;

mod utils;
fn main() -> Result<(), Box<dyn Error>> {
    utils::cli::unused_ips()?;
    println!("Hello, world!");
    Ok(())
}
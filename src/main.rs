use std::error::Error;

mod utils;
fn main() -> Result<(), Box<dyn Error>> {
    let ips_list = utils::cli::unused_ips();
    utils::file::write_unused_ips_file(ips_list)?;
    // println!("{:?}", unused_list);
    Ok(())
}
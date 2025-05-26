//! Get unused ips from cli then write then into corresponding file

use std::error::Error;
use std::fs::File;
use std::io::{Write};

pub fn write_unused_ips_file(ips_list: [Vec<String>;2]) -> Result<bool, Box<dyn Error>> {
    let mut unused_file = File::create("./unused_ips.txt")?;
    let mut used_file= File::create("./used_ips.txt")?;
    for ip in &ips_list[0] {
        writeln!(unused_file, "{}", ip)?
    }
    for ip in &ips_list[1] {
        writeln!(used_file, "{}", ip)?
    }
    Ok(true)
}


//tests
#[cfg(test)]
mod unit_tests {

    use crate::utils::file::write_unused_ips_file;

    #[test]
    fn test_write_unused_ips_file() {
        let ips_list: [Vec<String>;2] = [
            vec![
                "192.168.11.50".to_string(),
                "192.168.11.53".to_string(),
                "192.168.11.64".to_string()
                ],
            vec![
                "192.168.11.51".to_string(),
                "192.168.11.52".to_string(),
                "192.168.11.54".to_string(),
                "192.168.11.60".to_string()
            ]
        ];
        assert!(write_unused_ips_file(ips_list).is_ok());
    }
        
}

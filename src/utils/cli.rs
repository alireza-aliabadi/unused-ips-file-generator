//! Get user main subnet, number of ip ranges, start and end of each range from cli
//! Generate unused ips array

use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author="Alireza", version="0.1", about="unused ip scanner", long_about=None)]
struct CliArgs {
    #[arg(
        short,
        long = "main-subnet",
        help = "static subnets of your ip (i.e 192.168.1)"
    )]
    main_subnet: String,
    #[arg(
        short = 't',
        long = "ping-timeout",
        help = "define desire timeout duration for ping testing",
        default_value = "0.3"
    )]
    ping_timeout: String,
    #[arg(long="ranges-start-stop", value_name="RANGE_START_END", help="enter range start and end (i.e 52,62 65,75): ", num_args=1..)]
    range_start_end: Vec<String>,
}

fn unused_using_ping(subnet_ip: &str, timout: &str) -> bool {
    // check if ip is used or not
    let status = Command::new("ping")
        .args(["-c", "1", "-W", timout, subnet_ip])
        .output();
    match status {
        Ok(output) => !output.status.success(),
        Err(err) => panic!("{}", err),
    }
}

pub fn unused_ips() -> [Vec<String>; 2] {
    // Get: main subnet ip and number of ip ranges
    // Return: vector of ips or accused error
    let mut unused_ips_list: Vec<String> = vec![];
    let mut used_ips_list: Vec<String> = vec![];
    let args = CliArgs::parse();
    let timeout = args.ping_timeout;
    for range in &args.range_start_end {
        let se: Vec<i8> = range
            .split(',')
            .map(|s| s.parse::<i8>().expect("not valid number"))
            .collect();
        for ip in se[0]..=se[1] {
            let subnet = format!("{}.{}", &args.main_subnet, ip);
            if unused_using_ping(&subnet, &timeout) {
                unused_ips_list.push(subnet)
            } else {
                used_ips_list.push(subnet)
            }
        }
    }
    [unused_ips_list, used_ips_list]
}

// tests
#[cfg(test)]
mod unit_tests {
    use crate::utils::cli::unused_using_ping;

    #[test]
    fn test_unused_using_ping() {
        // test ping works properly by unused_using_ping function
        assert_eq!(
            unused_using_ping("8.8.8.8", "0.2"),
            false
        )
    }
}

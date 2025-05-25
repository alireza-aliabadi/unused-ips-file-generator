//! Get user main subnet, number of ip ranges, start and end of each range from cli
//! Generate unused ips array

use clap::Parser;
use std::process::{Command};
use std::error::Error;
// use rayon::prelude::*;
// use std::io::{self, Write, stdin, stdout};

#[derive(Parser, Debug)]
#[command(author="Alireza", version="0.1", about="unused ip scanner", long_about=None)]
struct CliArgs {
    #[arg(short, long="main-subnet", help="static subnets of your ip (i.e 192.168.1)")]
    main_subnet: String,
    //#[arg(short="irn", long="ip-range-numbers", help="number of ip range which you want to scan", default_value=1)]
    //ipRangeNumbers: i8,
    #[arg(value_name="RANGE_START_END", help="enter range start and end (i.e 52,62): ", num_args=1..)] 
    range_start_end: Vec<String>,
}

fn unused_using_ping(subnet_ip: &String) -> bool {
    // check if ip is used or not
    let status = Command::new("ping")
    .args(["-c","1",&subnet_ip])
    .output();
    match status {
        Ok(output) => !output.status.success(),
        Err(_) => {true}
    }
}

pub fn unused_ips() -> Result<Vec<String>, Box<dyn Error>> {
    // Get: main subnet ip and number of ip ranges
    // Return: vector of ips or accused error
    let mut unused_ips_list: Vec<String> = vec!();
    let args = CliArgs::parse();
    // let input = "";
    // let counter = 0;
    // while counter < args.ipRangeNumbers {
    //     print!("enter ip range start and end (i.e 57 63): ");
    //     stdout().flush().expect("Unable to flush stdout!");

    //     stdin().read_line(&mut input).expect("Unable to read input line");
    //     let range_start_end: [i8;2] = input.trim()
    //     .split_whitespace()
    //     .map(|s| s.parse().expect("Not valid number"))
    //     .collect();
    // }
    for range in &args.range_start_end {
        let se: Vec<i8> = range.split(',')
        .map(|s| s.parse::<i8>().expect("not valid number"))
        .collect();
        for ip in se[0]..=se[1] {
            let subnet = format!("{}.{}", &args.main_subnet, ip);
            if unused_using_ping(&subnet) {
                unused_ips_list.push(subnet)
            }
        }
    }
    Ok(unused_ips_list)
}
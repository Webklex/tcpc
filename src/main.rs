mod scanner;

use std::time::Duration;
use clap::{App, Arg};

fn main() {

    let arguments = App::new("tcpc")
        .version("0.0.1")
        .about("A simple, single threaded and minimalistic port checker.")
        .arg(
            Arg::new("target")
                .help("The target to scan")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("start_port")
                .help("Lowest port")
                .long("start_port")
                .short("s".parse().unwrap())
                .default_value("0"),
        )
        .arg(
            Arg::new("max_port")
                .help("Highest port")
                .long("max_port")
                .short("m".parse().unwrap())
                .default_value("65535"),
        )
        .arg(
            Arg::new("timeout")
                .help("Connection timeout in seconds")
                .long("timeout")
                .short("t".parse().unwrap())
                .default_value("10"),
        )
        .arg(
            Arg::new("delay")
                .help("Delay between two port checks in seconds")
                .long("delay")
                .short("d".parse().unwrap())
                .default_value("1"),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let target = arguments.value_of("target").unwrap();
    let timeout = arguments
        .value_of("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(10);
    let delay = arguments
        .value_of("delay")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(1);
    let start_port = arguments
        .value_of("start_port")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(1);
    let max_port = arguments
        .value_of("max_port")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(start_port + 1);

    let mut s = scanner::Scanner::new();

    s.set_target(target.to_string())
     .set_timeout(Duration::from_secs(timeout))
     .set_delay(Duration::from_secs(delay))
     .set_start_port(start_port)
     .set_max_port(max_port);

    s.start()
}

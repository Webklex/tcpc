mod scanner;

use std::time::Duration;
use clap::{App, Arg};

fn main() {
    let arguments = App::new("tcpc")
        .version("1.0.0")
        .about("A simple, single threaded and minimalistic port checker.")
        .arg(
            Arg::new("target")
                .help("The target to scan")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("File the output should be written to")
                .long("output")
                .short("o".parse().unwrap())
                .required(false)
                .default_value(""),
        )
        .arg(
            Arg::new("start_port")
                .help("Lowest port")
                .long("start_port")
                .short("s".parse().unwrap())
                .required(false)
                .default_value("0"),
        )
        .arg(
            Arg::new("max_port")
                .help("Highest port")
                .long("max_port")
                .short("m".parse().unwrap())
                .required(false)
                .default_value("65535"),
        )
        .arg(
            Arg::new("timeout")
                .help("Connection timeout in seconds")
                .long("timeout")
                .short("t".parse().unwrap())
                .required(false)
                .default_value("10"),
        )
        .arg(
            Arg::new("delay")
                .help("Delay between two port checks in seconds")
                .long("delay")
                .short("d".parse().unwrap())
                .required(false)
                .default_value("1"),
        )
        .arg(
            Arg::new("quiet")
                .help("Dont print anything into std")
                .long("quiet")
                .short("q".parse().unwrap())
                .required(false)
                .takes_value(false)
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .get_matches();

    let target = arguments.value_of("target").unwrap();
    let output = arguments.value_of("output").unwrap();
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
    s.set_quiet(arguments.is_present("quiet"))
        .set_target(target.to_string())
        .set_output_file(output.to_string())
        .set_timeout(Duration::from_secs(timeout))
        .set_delay(Duration::from_secs(delay))
        .set_start_port(start_port)
        .set_max_port(max_port);

    s.start()
}

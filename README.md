# TCPC - TCP Port Checker

A simple, single threaded and minimalistic port checker.

## Usage
```shell
USAGE:
    tcpc [OPTIONS] <target>

ARGS:
    <target>    The target to scan

OPTIONS:
    -s, --start_port <start_port>    Lowest port [default: 0]
    -m, --max_port <max_port>        Highest port [default: 65535]
    -d, --delay <delay>              Delay between two port checks in seconds [default: 1]
    -t, --timeout <timeout>          Connection timeout in seconds [default: 10]
    -V, --version                    Print version information
    -h, --help                       Print help information
```


## License
The MIT License (MIT). Please see [License File][link-license] for more information.

[link-license]: https://github.com/Webklex/tcpc/blob/master/LICENSE
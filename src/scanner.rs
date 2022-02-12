use std::thread;
use std::time::Duration;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

pub(crate) struct Scanner {
    target: String,
    start_port: u16,
    max_port: u16,
    timeout: Duration,
    delay: Duration,
}


impl Scanner {

    //
    // new
    // @Description: Build a new Scanner instance
    pub fn new() -> Self {
        Self {
            target: "".to_string(),
            start_port: 20,
            max_port: 22,
            timeout: Duration::from_secs(10),
            delay: Duration::from_secs(10),
        }
    }

    //
    // start
    // @Description: Start the scanner
    pub fn start(self) {
        // resolve the target
        if let Ok(socket_addresses) = format!("{}:0", self.target).to_socket_addrs() {
            let sockets: Vec<SocketAddr> = socket_addresses.collect();
            if sockets.is_empty() {
                return
            }
            let ip = sockets[0].ip();

            for port in self.start_port..self.max_port {

                // Check if a connection to a given socket can be established
                match TcpStream::connect_timeout(&SocketAddr::new(ip.clone(), port), self.timeout) {
                    Ok(_) => println!("{}", port),
                    Err(_) => ()
                }

                // if it isn't the last port to be checked, put the current thread to sleep
                if port < self.max_port {
                    thread::sleep(self.delay);
                }
            }
        }
    }

    //
    // set_target
    // @Description: Set the scanner target
    pub fn set_target(&mut self, t: String) -> &mut Self {
        self.target = t;
        self
    }

    //
    // set_start_port
    // @Description: Set the lowest port to be checked
    pub fn set_start_port(&mut self, sp: u16) -> &mut Self {
        self.start_port = sp;
        self
    }

    //
    // set_max_port
    // @Description: Set the highest port to be checked
    pub fn set_max_port(&mut self, mp: u16) -> &mut Self {
        self.max_port = mp;
        self
    }

    //
    // set_timeout
    // @Description: Set the scanner timeout. Time the requests has to finish until it gets canceled
    pub fn set_timeout(&mut self, t: Duration) -> &mut Self {
        self.timeout = t;
        self
    }

    //
    // set_delay
    // @Description: Set the scanner delay. Delay between two requests to the target
    pub fn set_delay(&mut self, d: Duration) -> &mut Self {
        self.delay = d;
        self
    }
}
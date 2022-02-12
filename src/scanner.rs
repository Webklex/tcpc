use std::{fs, thread};
use std::time::Duration;
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

pub(crate) struct Scanner {
    target: String,
    output_file: String,
    start_port: u16,
    max_port: u16,
    quiet: bool,
    timeout: Duration,
    delay: Duration,
}

struct OutputFile {
    path: String
}

impl OutputFile {

    //
    // new
    // @Description: Build a new OutputFile instance
    fn new(p: String) -> Self {
        Self {
            path: p,
        }
    }

    //
    // write_result
    // @Description: Build a new file handler
    fn build_file_handle(self) -> File {
        let mut file = OpenOptions::new();
        if Path::new(&self.path).exists() == false {
            file.create_new(true);
        }
        file.write(true)
            .append(true)
            .open(&self.path)
            .unwrap()
    }

    //
    // write_result
    // @Description: Append the given port to the current output file
    fn write_result(self, p: u16) {
        if let Err(e) = writeln!(self.build_file_handle(), "{}", p.to_string()) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

impl Scanner {

    //
    // new
    // @Description: Build a new Scanner instance
    pub fn new() -> Self {
        Self {
            target: "".to_string(),
            output_file: "output.txt".to_string(),
            start_port: 20,
            max_port: 22,
            quiet: false,
            timeout: Duration::from_secs(10),
            delay: Duration::from_secs(10),
        }
    }

    //
    // start
    // @Description: Start the scanner
    pub fn start(self) {
        self.prepare_output_file();
        self.scan();
    }

    //
    // prepare_output_file
    // @Description: Prepare the environment if an output file is wanted
    fn prepare_output_file(&self) {
        if self.output_file == "" {
            return;
        }

        let p = Path::new(&self.output_file);
        if p.exists() {
            self.remove_old_output_file();
        }else if p.parent().unwrap().exists() == false {
            match fs::create_dir_all(p.parent().unwrap()) {
                Ok(_) => {}
                Err(e) => eprintln!("Couldn't create directory: {}", e)
            }
        }
    }

    //
    // remove_old_output_file
    // @Description: Remove the old output file
    fn remove_old_output_file(&self) {
        match fs::remove_file(&self.output_file) {
            Ok(_) => {}
            Err(e) => eprintln!("Couldn't remove file: {}", e)
        }
    }

    //
    // scan
    // @Description: Scan the current target with the set port range
    pub fn scan(self) {
        // resolve the target
        if let Some(ip) = self.resolve_target() {
            for port in self.start_port..self.max_port {
                // Check if a connection to a given socket can be established
                match TcpStream::connect_timeout(&SocketAddr::new(ip.clone(), port), self.timeout) {
                    Ok(_) => self.handle_result(port),
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
    // handle_result
    // @Description: Handle the result if a port has been identified as open
    fn handle_result(&self, p: u16) {
        if self.quiet == false {
            println!("{}", p);
        }
        if self.output_file != "" {
            OutputFile::new(self.output_file.clone()).write_result(p)
        }
    }

    //
    // resolve_target
    // @Description: Get the resolved target address
    fn resolve_target(&self) -> Option<IpAddr> {
        if let Ok(socket_addresses) = format!("{}:0", self.target).to_socket_addrs() {
            let sockets: Vec<SocketAddr> = socket_addresses.collect();
            if sockets.is_empty() {
                return None
            }
            return Some(sockets[0].ip());
        }
        None
    }

    //
    // set_output_file
    // @Description: Set the scanner output file
    pub fn set_output_file(&mut self, of: String) -> &mut Self {
        self.output_file = of;
        self
    }

    //
    // set_target
    // @Description: Set the scanner target
    pub fn set_target(&mut self, t: String) -> &mut Self {
        self.target = t;
        self
    }

    //
    // set_quiet
    // @Description: Set the lowest port to be checked
    pub fn set_quiet(&mut self, q: bool) -> &mut Self {
        self.quiet = q;
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
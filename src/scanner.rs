use std::{fs, thread};
use std::time::Duration;
use std::path::Path;
use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::fs::{File, OpenOptions};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ScannerError {
    #[error("The given port range is invalid.")]
    InvalidPortRange,
    #[error("The given target is invalid.")]
    InvalidTarget,

    #[error("Couldn't write to file: {error}")]
    WriteFileFailed { error: String },
    #[error("Couldn't open file: {error}")]
    OpenFileFailed { error: String },
    #[error("Couldn't remove file: {error}")]
    RemoveFileFailed { error: String },
    #[error("Couldn't create directory: {error}")]
    CreateDirectoryFailed { error: String },
}

pub(crate) struct Scanner {
    target: String,
    output_file: String,
    start_port: u16,
    max_port: u16,
    quiet: bool,
    display_socket: bool,
    timeout: Duration,
    delay: Duration,
}

struct OutputFile {
    path: String,
}

impl OutputFile {
    /// Build a new OutputFile instance
    fn new(p: String) -> Self {
        Self {
            path: p,
        }
    }

    /// Build a new file handler
    fn build_file_handle(self) -> Result<File, ScannerError> {
        let mut file = OpenOptions::new();
        if Path::new(&self.path).exists() == false {
            file.create_new(true);
        }
        match file.write(true).append(true).open(&self.path) {
            Ok(file) => Ok(file),
            Err(e) => Err(ScannerError::OpenFileFailed {
                error: e.to_string()
            })
        }
    }

    /// Append the given port to the current output file
    fn write_result(self, s: String) -> Result<(), ScannerError> {
        match writeln!(self.build_file_handle()?, "{}", s) {
            Ok(_) => Ok(()),
            Err(e) => Err(ScannerError::WriteFileFailed {
                error: e.to_string(),
            })
        }
    }
}

impl Scanner {
    /// Build a new Scanner instance
    pub fn new() -> Self {
        Self {
            target: "".to_string(),
            output_file: "output.txt".to_string(),
            start_port: 0,
            max_port: 65535,
            quiet: false,
            display_socket: false,
            timeout: Duration::from_secs(10),
            delay: Duration::from_secs(10),
        }
    }

    /// Start the scanner
    pub fn start(self) -> Result<(), ScannerError> {
        self.validate()?.prepare_output_file()?.scan()
    }

    /// Validate the current Scanner instance
    pub fn validate(&self) -> Result<&Self, ScannerError> {
        if self.start_port > self.max_port {
            return Err(ScannerError::InvalidPortRange);
        }
        Ok(self)
    }

    /// Prepare the environment if an output file is wanted
    fn prepare_output_file(&self) -> Result<&Self, ScannerError> {
        if self.output_file == "" {
            return Ok(self);
        }

        let p = Path::new(&self.output_file);
        if p.exists() {
            return self.remove_old_output_file();
        }
        self.create_output_directory(p)
    }

    /// Create the output directory from a given path
    fn create_output_directory(&self, p: &Path) -> Result<&Self, ScannerError> {
        let parent = p.parent().unwrap();
        if parent.exists() == false {
            return match fs::create_dir_all(parent) {
                Ok(_) => Ok(self),
                Err(e) => Err(ScannerError::CreateDirectoryFailed { error: e.to_string() })
            };
        }
        Ok(self)
    }

    /// Remove the old output file
    fn remove_old_output_file(&self) -> Result<&Self, ScannerError> {
        match fs::remove_file(&self.output_file) {
            Ok(_) => Ok(self),
            Err(e) => Err(ScannerError::RemoveFileFailed { error: e.to_string() })
        }
    }

    /// Scan the current target with the set port range
    pub fn scan(&self) -> Result<(), ScannerError> {
        // resolve the target
        let ip = self.resolve_target()?;
        let mut port = self.start_port;

        //for port in self.start_port..self.max_port {
        while port <= self.max_port {
            // Check if a connection to a given socket can be established
            match TcpStream::connect_timeout(&SocketAddr::new(ip.clone(), port), self.timeout) {
                Ok(_) => self.handle_result(port)?,
                Err(_) => ()
            }

            // if it isn't the last port to be checked, put the current thread to sleep
            if port < self.max_port {
                thread::sleep(self.delay);
            }
            port += 1
        }
        Ok(())
    }

    /// Handle the result if a port has been identified as open
    fn handle_result(&self, p: u16) -> Result<(), ScannerError> {
        let mut result = p.to_string();
        if self.display_socket {
            result = format!("{}:{}", self.target, result);
        }
        if self.quiet == false {
            println!("{}", result);
        }
        if self.output_file != "" {
            OutputFile::new(self.output_file.clone()).write_result(result)?
        }
        Ok(())
    }

    /// Get the resolved target address
    fn resolve_target(&self) -> Result<IpAddr, ScannerError> {
        if let Ok(socket_addresses) = format!("{}:0", self.target).to_socket_addrs() {
            let sockets: Vec<SocketAddr> = socket_addresses.collect();
            if sockets.is_empty() {
                return Err(ScannerError::InvalidTarget);
            }
            return Ok(sockets[0].ip());
        }
        Err(ScannerError::InvalidTarget)
    }

    /// Set the scanner output file
    pub fn set_output_file(&mut self, of: String) -> &mut Self {
        self.output_file = of;
        self
    }

    /// Set the scanner target
    pub fn set_target(&mut self, t: String) -> &mut Self {
        self.target = t;
        self
    }

    /// Set the quiet flag
    pub fn set_quiet(&mut self, q: bool) -> &mut Self {
        self.quiet = q;
        self
    }

    /// Set the display socket flag
    pub fn set_display_socket(&mut self, ds: bool) -> &mut Self {
        self.display_socket = ds;
        self
    }

    /// Set the lowest port to be checked
    pub fn set_start_port(&mut self, sp: u16) -> &mut Self {
        self.start_port = sp;
        self
    }

    /// Set the highest port to be checked
    pub fn set_max_port(&mut self, mp: u16) -> &mut Self {
        self.max_port = mp;
        self
    }

    /// Set the scanner timeout. Time the requests has to finish until it gets canceled
    pub fn set_timeout(&mut self, t: Duration) -> &mut Self {
        self.timeout = t;
        self
    }

    /// Set the scanner delay. Delay between two requests to the target
    pub fn set_delay(&mut self, d: Duration) -> &mut Self {
        self.delay = d;
        self
    }
}
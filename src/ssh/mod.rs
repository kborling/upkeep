use dirs::home_dir;
use ssh2::{MethodType, Session};
use ssh2_config::{HostParams, SshConfig};
use std::env::args;
use std::fs;
use std::io::BufReader;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::time::Duration;

use std::rc::Rc;
use std::result::Result;

pub struct SSHClient {

}

impl SSHClient {
    // pub fn new(ssh: Arc<tokio::sync::Mutex<App>>) -> Self {
    //     Self { ssh }
    // }
    /*
     * Read and parse the specified
     * SSH configuration file and
     * build a list of Host options
     */
    // pub fn get_host_options(p: &Path) -> Vec<String> {
    pub fn get_host_options() -> Vec<String> {

        let mut p = home_dir().expect("Failed to get Home directory.");
        p.extend(Path::new(".ssh/config"));

        let mut options = vec![];
        let contents = fs::read_to_string(&p).expect("Something went wrong reading the file");

        for line in contents.lines() {
            match line.split_whitespace().next() {
                Some(x) => {
                    if x == "Host" {
                        let connection = line.split_whitespace().last().unwrap_or("").to_string();
                        options.push(connection);
                    }
                }
                None => (),
            }
        }
        options
    }

    /*
     * Read and parse the specified
     * SSH configuration file.
     */
    fn read_config(p: &Path) -> SshConfig {
        let mut reader = match fs::File::open(p) {
            Ok(f) => BufReader::new(f),
            Err(err) => panic!("Could not open file '{}': {}", p.display(), err),
        };
        match SshConfig::default().parse(&mut reader) {
            Ok(config) => config,
            Err(err) => panic!("Failed to parse configuration: {}", err),
        }
    }

    /*
     * Connect to the specified host using
     * the host parameters.
     */
    fn connect(host: &str, params: &HostParams) {
        // Resolve host
        let host = match params.host_name.as_deref() {
            Some(h) => h,
            None => host,
        };
        let port = match params.port {
            None => 22,
            Some(p) => p,
        };
        let host = match host.contains(':') {
            true => host.to_string(),
            false => format!("{}:{}", host, port),
        };
        println!("Connecting to host {}...", host);
        let socket_addresses: Vec<SocketAddr> = match host.to_socket_addrs() {
            Ok(s) => s.collect(),
            Err(err) => {
                panic!("Could not parse host: {}", err);
            }
        };
        let mut tcp: Option<TcpStream> = None;
        // Try addresses
        for socket_addr in socket_addresses.iter() {
            match TcpStream::connect_timeout(
                socket_addr,
                params.connect_timeout.unwrap_or(Duration::from_secs(30)),
            ) {
                Ok(stream) => {
                    println!("Established connection with {}", socket_addr);
                    tcp = Some(stream);
                    break;
                }
                Err(_) => continue,
            }
        }
        // If stream is None, return connection timeout
        let stream: TcpStream = match tcp {
            Some(t) => t,
            None => {
                panic!("No suitable socket address found; connection timeout");
            }
        };
        let mut session: Session = match Session::new() {
            Ok(s) => s,
            Err(err) => {
                panic!("Could not create session: {}", err);
            }
        };
        // Configure session
        SSHClient::configure_session(&mut session, params);
        // Connect
        session.set_tcp_stream(stream);
        if let Err(err) = session.handshake() {
            panic!("Handshake failed: {}", err);
        }
        // Get username
        let username = match params.user.as_ref() {
            Some(u) => {
                println!("Using username '{}'", u);
                u.clone()
            }
            None => SSHClient::read_secret("Username: "),
        };
        let password = SSHClient::read_secret("Password: ");
        if let Err(err) = session.userauth_password(username.as_str(), password.as_str()) {
            panic!("Authentication failed: {}", err);
        }
        if let Some(banner) = session.banner() {
            println!("{}", banner);
        }
        println!("Connection OK!");
        if let Err(err) = session.disconnect(None, "mandi mandi!", None) {
            panic!("Disconnection failed: {}", err);
        }
    }

    /*
     * Configure the SSH session before
     * making the connection.
     */
    fn configure_session(session: &mut Session, params: &HostParams) {
        println!("Configuring session...");
        if let Some(compress) = params.compression {
            println!("compression: {}", compress);
            session.set_compress(compress);
        }
        if params.tcp_keep_alive.unwrap_or(false) && params.server_alive_interval.is_some() {
            let interval = params.server_alive_interval.unwrap().as_secs() as u32;
            println!("keepalive interval: {} seconds", interval);
            session.set_keepalive(true, interval);
        }
        // algos
        if let Some(algos) = params.kex_algorithms.as_deref() {
            if let Err(err) = session.method_pref(MethodType::Kex, algos.join(",").as_str()) {
                panic!("Could not set KEX algorithms: {}", err);
            }
        }
        if let Some(algos) = params.host_key_algorithms.as_deref() {
            if let Err(err) = session.method_pref(MethodType::HostKey, algos.join(",").as_str()) {
                panic!("Could not set host key algorithms: {}", err);
            }
        }
        if let Some(algos) = params.ciphers.as_deref() {
            if let Err(err) = session.method_pref(MethodType::CryptCs, algos.join(",").as_str()) {
                panic!("Could not set crypt algorithms (client-server): {}", err);
            }
            if let Err(err) = session.method_pref(MethodType::CryptSc, algos.join(",").as_str()) {
                panic!("Could not set crypt algorithms (server-client): {}", err);
            }
        }
        if let Some(algos) = params.mac.as_deref() {
            if let Err(err) = session.method_pref(MethodType::MacCs, algos.join(",").as_str()) {
                panic!("Could not set MAC algorithms (client-server): {}", err);
            }
            if let Err(err) = session.method_pref(MethodType::MacSc, algos.join(",").as_str()) {
                panic!("Could not set MAC algorithms (server-client): {}", err);
            }
        }
    }

    /*
     * Read password input from Stdin
     */
    fn read_secret(prompt: &str) -> String {
        rpassword::read_password_from_tty(Some(prompt)).expect("Failed to read from stdin")
    }
}

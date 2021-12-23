extern crate clap;
extern crate dirs;
use clap::{App, Arg};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
struct Connection {
    host: String,
    host_name: String,
    user: String,
    password: String,
    port: u8,
    identity_file: String,
}

impl Default for Connection {
    fn default() -> Connection {
        Connection {
            host: "".to_string(),
            host_name: "".to_string(),
            user: "".to_string(),
            password: "".to_string(),
            port: 0,
            identity_file: "".to_string(),
        }
    }
}

fn main() {
    let matches = App::new("Upkeep")
        .version("v0.1")
        .author("Kevin Borling <kborling@protonmail.com>")
        .about("Keep your websites up-to-date with ease")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("SSH config file location. Defaults to ~/.ssh/config")
                .takes_value(true),
        )
        .get_matches();

    // Get user's ssh config path if supplied
    // Default to ssh config in user's home directory
    let config_path: PathBuf = match matches.value_of("config") {
        Some(path) => Path::new(path).to_path_buf(),
        None => dirs::home_dir().unwrap().join(".ssh/config"),
    };

    // TODO: Remove after testing
    assert_eq!(config_path, PathBuf::new().join("/home/kevin/.ssh/config"));
    // TODO: Ensure file exists

    // println!("Value for config: {:?}", config_path);

    let contents = get_file_contents(config_path.as_path());
    // Output for debugging
    // println!("{}", contents);

    // TODO: Get Host line from config
    let connections = get_connections(contents);
    println!("Value for connections: {:?}", connections);
    // TODO: List connection prompt to user for selection
}

/*
 * Read the provided config file
 * and return the contents.
 */
fn get_file_contents(config: &Path) -> String {
    let contents = fs::read_to_string(&config).expect("Something went wrong reading the file");

    contents
}

/*
 * Parse the contents of the SSH
 * config file to build a list
 * of all connections.
 */
fn get_connections(contents: String) -> Vec<Connection> {
    let mut connections = vec![];

    // TODO: Is there a better way to parse contents into a struct?
    let mut connection = Connection::default();

    // Parse SSH connection values on each line
    // into Connection struct
    for line in contents.lines() {
        match line.split_whitespace().next() {
            Some(x) => {
                if x == "Host" {
                    if !connection.host.is_empty() {
                        // Add previous connection
                        connections.push(connection);
                        // Reset connection data
                        connection = Connection::default();
                    }
                    println!("{}", line);
                    connection.host = line.split_whitespace().last().unwrap_or("").to_string();
                } else if x == "HostName" {
                    connection.host_name = line.split_whitespace().last().unwrap_or("").to_string();
                } else if x == "User" {
                    connection.user = line.split_whitespace().last().unwrap_or("").to_string();
                } else if x == "Password" {
                    connection.password = line.split_whitespace().last().unwrap_or("").to_string();
                } else if x == "Port" {
                    // connection.port = line.split_whitespace().last().unwrap_or("").to_string();
                } else if x == "IdentityFile" {
                    connection.identity_file = line.split_whitespace().last().unwrap_or("").to_string();
                }
            }
            None => (),
        }
    }
    connections
}

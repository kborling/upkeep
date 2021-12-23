extern crate clap;
extern crate dirs;
use clap::{App, Arg};
use std::fs;
// use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
struct Connection {
    host: String,
    host_name: String,
    user: String,
    port: u8,
    identity_file: String,
}

fn main() {
    let matches = App::new("Upkeep")
        .version("v0.1")
        .author("Kevin Borling <kborling@protonmail.com>")
        .about("Keep your websites up-to-date with ease")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .help("SSH config file location. Defaults to ~/.ssh/config")
             .takes_value(true)
             )
        .get_matches();

    // Get user's ssh config path if supplied
    // Default to ssh config in user's home directory
    let config_path: PathBuf = match matches.value_of("config") {
        Some(path) => Path::new(path).to_path_buf(),
        None => dirs::home_dir().unwrap().join(".ssh/config"),
    };

    println!("Value for config: {:?}", config_path);

    let contents = get_file_contents(config_path.as_path());
    // Output for debugging
    println!("With text:\n{}", contents);

    // TODO: Get Host line from config
    let connections = get_connections(contents);
}

/*
 * Parse the contents of the SSH
 * config file to build a list
 * of all connections.
 */
fn parse_configuration_file() {
}

/*
 * Read the provided config file
 * and return the contents.
 */
fn get_file_contents(config: &Path) -> String {
    let contents = fs::read_to_string(&config)
        .expect("Something went wrong reading the file");

    contents
}

/*
 * Parse the contents of the SSH
 * config file to build a list
 * of all connections.
 */
fn get_connections(contents: String) -> Vec<Connection> {

    // TODO: Parse Connections
    let hosts = Vec::new();
    hosts
}

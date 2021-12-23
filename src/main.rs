extern crate clap;
use clap::{App, Arg};
use std::fs;

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

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("/home/kevin/.ssh/config");
    println!("Value for config: {}", config);


    let contents = get_file_contents(config);
    // Output for debugging
    println!("With text:\n{}", contents);

    // TODO: Get Host line from config
    let connections = get_connections(contents);
}

/*
 * Read the provided config file
 * and return the contents.
 */
fn get_file_contents(config: &str) -> String {
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

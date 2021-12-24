use clap::{App, Arg};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::io::BufReader;
use ssh2_config::SshConfig;

fn main() {
    // TODO: Replace clap with tui
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

    let mut reader = BufReader::new(fs::File::open(config_path).expect("Could not open configuration file"));
    let config = SshConfig::default().parse(&mut reader).expect("Failed to parse configuration");
    println!("{:?}", config);
}
